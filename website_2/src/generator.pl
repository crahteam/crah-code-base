my $string = $ARGV[0];
$string =~ s/$(\w+)/replace_word($1)/ge;
print $string;

sub replace_word {
    my $matched_word = shift;
    if ($matched_word eq 'ADDRESS') {
        return "www.crah.app";
    }
    # Default replacement if no match is found
    return "";
}