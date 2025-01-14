# [1] Subscription

Packets related to subscriptions.

- Group ID: [1]
- Member ID length: 1

## [0] Subscribe

Request subscribe to a resource. If an echo is not supplied, no response will be given.

#### With no additional info

```
1,1,0,[channel]
```

#### With doas only

```
1,1,0,[channel],0,[doas]
```

#### With echo only

```
1,1,0,[channel],0,0,[echo]
```

#### With echo and doas

```
1,1,0,[channel],0,[doas],0,[echo]
```

## [1] Unsubscribe

Request unsubscribe to a resource. If an echo is not supplied, no response will be given.

#### With no additional info

```
1,1,1,[channel]
```

#### With doas only

```
1,1,1,[channel],0,[doas]
```

#### With echo only

```
1,1,1,[channel],0,0,[echo]
```

#### With echo and doas

```
1,1,1,[channel],0,[doas],0,[echo]
```

## [2] Subscribed

Response to successful subscription.

```
1,1,2,[echo]
```

## [3] Unsubscribed

Response to successful unsubscription.

```
1,1,3,[echo]
```

## [4] NotFound

Response for not subscribed because not found.

```
1,1,4,[echo]
```

## [5] Denied

Response for not subscribed because rejected.

```
1,1,5,[echo]
```
