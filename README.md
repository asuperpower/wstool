# wstool
## NAME
  wstool - simulate websocket traffic

## SYNOPSIS
  wstool [OPTIONS ... ] [PATH]

## DESCRIPTION
  Connect to PATH and display to standard output.

  With a FILE, it will read it line by line and send it,
  waiting based on rules before sending the next one

  -a --arg=ARG1,ARG2,ARG3
    Inject arguments into file (if read in). Will replace 
    $1,$2,$3 in the read in file with ARG1 ARG2 and ARG3

  -f --file <filepath>
    File to read in

#### File structure
Each line represents a message followed by a rule of when
to send the next message. They are seperated by whitespace.
For example:
```
Hello   10s
World   W
!       .
```

The file supports whitespace in the message as long as it
is not trailing. This means you could do the following:
```
Hello   world!!!  10s
!                 E
```

This would send out "Hello   world!!!" on the websocket,
wait 10 seconds and then send "!".

Obviously this has its limitations (mainly a message cannot
END with a whitespace) but it is the only currently
supported mode. Someone is welcome to extend this with a
command line switch.
