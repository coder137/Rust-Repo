```mermaid
---
title: Traffic Light
---
stateDiagram-v2
    [*] --> Red
    Red --> Yellow: RedDone
    Yellow --> Green: YellowDone
    Green --> Red: GreenDone
```
