// The standard structure for identifying things in Eartha's Frontier
// Depending on the system archtecture, it uses 32-bit numbers or 64-bit numbers

mod identifier {
    struct EFGIDAuto {
        local_id: usize,
        parent_id: usize,
        system_level: usize
    }

    struct EFGID8 {
        local_id: u8,
        parent_id: u8,
        system_level: u8
    }

    struct EFGID16 {
        local_id: u16,
        parent_id: u16,
        system_level: u16
    }

    struct EFGID32 {
        local_id: u32,
        parent_id: u32,
        system_level: u32
    }

    struct EFGID64 {
        local_id: u64,
        parent_id: u64,
        system_level: u64
    }

    struct EFGID128 {
        local_id: u128,
        parent_id: u128,
        system_level: u128
    }

    enum EFGID {
        EFGIDAuto, EFGID8, EFGID16, EFGID32, EFGID64, EFGID128
    }

    fn return_efgid_options() -> [&str] {
        ["auto", "8-bit", "16-bit", "32-bit", "64-bit", "128-bit"]
    }

    fn return_efgid(choice: &str) -> EFGID {
        match choice {
            "auto" => EFGIDAuto, "8-bit", "16-bit", "32-bit", "64-bit", "128-bit"]
        } 
    }
}

struct EFSystem<T> {
    id_counter: T
}

fn create_system() -> EFSystem {
    // Choose an autoincrementor
}

struct EFObject<T> {
    gid: EFGID,
    object: T,
    object_type: String
}