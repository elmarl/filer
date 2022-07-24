A small app written in Rust intended to easily share files over local network between two computers.

The receiving side has to open the app and listen for connections using the following command:

(Directly from vscode) cargo run -- 

To send a file on the same pc (change ip respectively if another pc):

(Directly from vscode) cargo run -- --ip 127.0.0.1 -f ./file.txt
