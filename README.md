
# Porz

**Porz** is a lightweight port scanner to check if a port is open or closed
It's a tiny project

You can check a single or a list of ports using this
## Usage

You can run **Porz** with the following command-line options:

```
porz -a "ADDRESS" -s "START PORT" -e "END PORT"
```

### Options

- `-a "ADDRESS"`: Specify the target address (default: `127.0.0.1`).
- `-s "START PORT"`: Define the starting port number for scanning.
- `-e "END PORT"`: Set the ending port number for scanning.

## Example

To scan ports 8080 to 8090 on the local machine, use:

```
porz -a "127.0.0.1" -s "8080" -e "8090"
```

