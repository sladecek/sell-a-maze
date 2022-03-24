%builtins output pedersen range_check

# This program validates that there is a solution to a maze.

from starkware.cairo.common.serialize import serialize_word
from starkware.cairo.common.hash_chain import hash_chain
from starkware.cairo.common.cairo_builtins import HashBuiltin

func main{output_ptr : felt*, pedersen_ptr : HashBuiltin*, range_check_ptr}():
    alloc_locals

    # maze structure -------------------------------------------
    # number od rooms
    local n_rooms
    # number of walls
    local n_walls
    # array of unique primes identifying the rooms
    local room_primes : felt*
    # array of room primes multiplied for each wall
    local wall_products : felt*
    
    # maze instance --------------------------------------------
    # 1 if the wall is closed, 0 if there is a door in the wall
    local is_door_closed : felt*

    # room solution --------------------------------------------
    # path length (number of rooms in the path) 
    local n_solution    
    # array af alternating room and wall indices on the path
    # room index, wall index, room index ...
    # contains 2 * n_solution - 1 elements
    local solution : felt*


    %{
        # Read maze structure ---------------------------------------
        f = open("maze.mas")
        a = f.readlines()
        ids.n_rooms = int(a[0])
        ids.n_walls = int(a[1])
        assert(len(a) == 2 + ids.n_rooms + ids.n_walls)

        offset = 2
        ids.room_primes = room_primes = segments.add()
        # hash_chain() requires the length to be stored as the first element
        memory[room_primes] = ids.n_rooms
        for i in range(ids.n_rooms):
            memory[room_primes + i + 1] = int(a[i+offset])

        offset += ids.n_rooms
        ids.wall_products = wall_products = segments.add()
        memory[wall_products] = ids.n_walls
        for i in range(ids.n_walls):
            memory[wall_products + i + 1] = int(a[i+offset])

        # Read maze instance ---------------------------------------
        f = open("maze.mai")
        a = f.readlines()
        assert(len(a) == ids.n_walls)
        ids.is_door_closed = is_door_closed = segments.add()
        memory[is_door_closed] = ids.n_walls
        for i in range(ids.n_walls):
            is_closed = int(a[i])
            memory[is_door_closed + i + 1] = is_closed
            assert (is_closed == 0 or is_closed == 1)

        # Read maze solution (path) --------------------------------
        f = open("maze.map")
        a = f.readlines()
        ids.solution = solution = segments.add()        
        ids.n_solution = int(a[0])
        offset = 1
        assert(len(a) ==  ids.n_solution * 2 - 1 + 1)
        for i in range(ids.n_solution * 2 - 1):
            memory[solution + i] = int(a[i+offset])
                
    %}


    # First room of the solution  must be the starting room of the maze.
    assert [solution] = 0
    
    # Last room of the solution must be the target room of the maze.
    assert [solution + n_solution * 2 - 1 - 1] = n_rooms - 1

    # Validate path
    assert_path(sol=solution, size=n_solution, room_primes = room_primes+1, wall_products = wall_products+1, is_door_closed = is_door_closed+1 )

    # Compute and print hashes of input data
    let (room_hash) = hash_chain{hash_ptr=pedersen_ptr}(room_primes)
    let (wall_hash) = hash_chain{hash_ptr=pedersen_ptr}(wall_products)
    let (instance_hash) = hash_chain{hash_ptr=pedersen_ptr}(is_door_closed)     
    serialize_word(room_hash)
    serialize_word(wall_hash)    
    serialize_word(instance_hash)

    return ()
end

func assert_path(sol: felt*, size, room_primes: felt*, wall_products: felt*, is_door_closed: felt*) -> ():
    alloc_locals

    if size == 1:
       return ()
    end

    local r1 = [sol]
    local w = [sol+1]
    local r2 = [sol+2]

    # No need to validate wall and room indices range. They are in range
    # because otherwise the cairo engine would refuse to execute this
    # program with the error message 'Unknown value for memory cell at
    # address'.

    # Wall is open in the instance.
    assert [is_door_closed + w] = 0

    # Path is continuous.
    assert [room_primes + r1] * [room_primes + r2] = [wall_products + w]

    assert_path(sol = sol+2, size = size-1, room_primes = room_primes, wall_products = wall_products, is_door_closed = is_door_closed )
    return ()
end
