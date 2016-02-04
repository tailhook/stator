======
Stator
======

:Status: Proof of Concept

Goal: fast library that does asyncrhonous stuff for scripting languages
(python)

Challenges:

* Scripting languages do everything dynamically, so we can't use normal rotor
  composition of state machines. Should either use virtual dispatch or have a
  type that has all protocols at once.

