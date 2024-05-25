fn main() {
    // Initialize a MaybeUninit<statvfs> struct to hold the file system information
    let mut stat: std::mem::MaybeUninit<libc::statvfs> = std::mem::MaybeUninit::uninit();

    // Call the statvfs system call to retrieve file system statistics for the root directory (/)
    // The result is stored in the stat variable
    unsafe {
        if libc::statvfs(b"/\0".as_ptr() as *const libc::c_char, stat.as_mut_ptr()) != 0 {
            eprintln!("Failed to get disk usage for root directory");
            return;
        }
    }

    // Assume that the stat variable is initialized and get a reference to it
    let stat = unsafe { stat.assume_init() };

    // Extract relevant information from the statvfs struct
    let block_size = stat.f_bsize as u64;
    let blocks_available = stat.f_blocks;
    let blocks_used = stat.f_blocks - stat.f_bfree;

    // Calculate the total disk size and used disk size
    let total_size = blocks_available * block_size;
    let used_size = blocks_used * block_size;

    // Calculate the disk usage percentage
    let disk_usage_percentage = (used_size as f64 / total_size as f64) * 100.0;

    // Calculate the available disk space by subtracting the used space from the total space
    let available_size = stat.f_bavail as u64 * block_size;

    // Print the available disk space and disk usage percentage
    println!("Available Space: {}", human_readable_size(available_size));
    println!("Disk Usage: {:.2}%", disk_usage_percentage);
}

/// Converts a disk size in bytes to a human-readable format (e.g., "1.23 GiB")
fn human_readable_size(size: u64) -> String {
    let units = ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];
    let mut size_f64 = size as f64;
    let mut unit_index = 0;

    // Divide the size by 1024 until it's less than 1024 or we reach the largest unit
    while size_f64 >= 1024.0 && unit_index < units.len() - 1 {
        size_f64 /= 1024.0;
        unit_index += 1;
    }

    // Format the size with two decimal places and append the unit
    format!("{:.2}", size_f64) + " " + units[unit_index]
}
