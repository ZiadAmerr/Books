#!/usr/bin/perl
use strict;
use warnings;
use Time::HiRes qw(time);
use JSON;

sub parse_vcd_file {
    my ($file_name) = @_;
    print "(info) Parsing file: $file_name\n";
    my $start = time;
    open my $fh, '<', $file_name or die "Cannot open file '$file_name': $!";
    my $event_count = 0;
    while (my $line = <$fh>) {
        chomp $line;
        if (($line =~ /^0/ || $line =~ /^1/) && $line =~ /!$/) {
            $event_count++;
        }
    }
    close $fh;
    my $duration_ms = int((time - $start) * 1000);
    return ($event_count, $duration_ms);
}

sub parse_all_vcd_files {
    my ($vcd_dir) = @_;
    my @files;
    opendir(my $dh, $vcd_dir) or die "Cannot open directory '$vcd_dir': $!";
    while (my $entry = readdir($dh)) {
        next if $entry eq '.' or $entry eq '..';
        my $subdir = "$vcd_dir/$entry";
        next unless -d $subdir && $entry =~ /^\d+$/;
        opendir(my $sub_dh, $subdir) or next;
        while (my $file = readdir($sub_dh)) {
            next if $file eq '.' or $file eq '..';
            my $file_path = "$subdir/$file";
            push @files, $file_path if -f $file_path && $file =~ /\.vcd$/;
        }
        closedir $sub_dh;
    }
    closedir $dh;
    return \@files;
}

sub parse_vcd_files {
    my ($paths_ref) = @_;
    my %results;
    for my $file (@$paths_ref) {
        my ($events, $duration) = parse_vcd_file($file);
        $results{$file} = [$events, $duration];
    }
    return \%results;
}

unless (caller) {
    my $vcd_directory = $ARGV[0] // '.';
    print "Parsing all VCD files in directory: $vcd_directory\n";
    my $start_time = time();
    my $files_ref = parse_all_vcd_files($vcd_directory);
    my $results = parse_vcd_files($files_ref);
    my $end_time = time();
    printf("Total time taken to parse all VCD files: %.2f seconds\n", $end_time - $start_time);
    print "Parsed VCD files JSON:\n", encode_json($results), "\n";
}
