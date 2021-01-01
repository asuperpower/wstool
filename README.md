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
Hello   10s
World   W
!       .

Obviously this has its limitations but is the only
currently supported mode. Someone is welcome to extend this
with a command line switch.
