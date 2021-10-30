# tonic-grpc

```
$ grpcurl -plaintext -d '{"name":"foo"}' localhost:8080 helloworld.Greeter.SayHello
{
  "message": "Hello foo!"
}
```
