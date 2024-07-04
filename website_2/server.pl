#!/usr/bin/perl
use strict;
use warnings;
use Plack::Builder;
use Plack::App::File;

my $root = '/mnt/usb/scrappy/html_parser/';  # Update this with the actual path to your files

my $app = sub {
    my $env = shift;
    if ($env->{PATH_INFO} =~ m{^/(?:js|css|img|wasm)/}) {
        # Serve static files directly without redirection
        return Plack::App::File->new({ root => $root })->to_app($env);
    } else {
        # Handle other requests by serving the index.html file
        my $file = 'public/index.html';
        open(my $fh, '<', $file) or return [500, ['Content-Type' => 'text/plain'], ['Internal Server Error']];
        my $content = do { local $/; <$fh> };
        close($fh);
        return [200, ['Content-Type' => 'text/html'], [$content]];
    }
};

builder {
    mount '/' => $app;
};