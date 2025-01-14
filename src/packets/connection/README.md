# [0] Connection

Packets related to connections with the ccanvas server.

- Group ID: [1]
- Member ID length: 1

## [0] ReqConn

Request to create a new connection. If a socket path is not supplied then the connection will not be attached.

#### With socket

```
1,0,0,[label.utf8],0,[socket.utf8],0,[echo]
```

#### Without socket

```
1,0,0,[label.utf8]
```

## [1] ApprConn

Response if an attached connection is approved.

```
1,0,1,[echo]
```

## [2] RejConn

Response if an attached connection is rejected.

```
1,0,2,[echo]
```

## [3] Terminate

Clients or servers receiving this should exit immediately.

```
1,0,3
```
