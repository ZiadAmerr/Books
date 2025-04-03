use strict;
use warnings;
use FFI::Platypus;
use JSON;
use Data::Dumper;
# Import high resolution time
use Time::HiRes qw(time);

# Create a new FFI::Platypus instance
my $ffi = FFI::Platypus->new(api => 1);

# Load the compiled Rust shared library
$ffi->lib("/Users/ziad/Desktop/ML/Books/ProgrammingRust/projs/vcd/target/release/libvcd.dylib");

# Declare the array type (array of strings)
$ffi->type('string[]' => 'string_array');

# Attach the Rust functions
$ffi->attach( parse_vcd_files_from_perl => ['string_array', 'size_t'] => 'string' );
$ffi->attach( free_c_string => ['string'] => 'void' );

my $vcd_files_root = "/Users/ziad/Desktop/ML/Books/ProgrammingRust/projs/vcd/vcd_files";
my @vcd_file_sizes = qw( 100 1000 10000 100000 1000000 10000000);
my $num_files_per_size = 20;
my @files;
foreach my $size (@vcd_file_sizes) {
    for (my $i = 1; $i <= $num_files_per_size; $i++) {
        my $file_path = "$vcd_files_root/$size/vcd_output_${size}_$i.vcd";
        push @files, $file_path;
    }
}

my $start_time;
my $end_time;
my $time_taken;

$start_time = time();
my $json = parse_vcd_files_from_perl(\@files, scalar @files);  
printf("Time taken to parse %d files: %.2f seconds\n", scalar @files, time() - $start_time);


$start_time = time();
my $results = decode_json($json);
$end_time = time();
printf("Time taken to decode JSON: %.2f seconds\n", time() - $start_time);

# print Dumper($results);

# Free the memory returned by Rust after decoding JSON
# free_c_string($json);
