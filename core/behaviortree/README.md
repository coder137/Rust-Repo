# BehaviorTree

# Example

## Traffic Light

```mermaid
---
title: Traffic Light
---
graph TB
    Parallel --> WaitForever
    Parallel --> Sequence
    Sequence --> Red
    Sequence --> Yellow
    Sequence --> Green
```
