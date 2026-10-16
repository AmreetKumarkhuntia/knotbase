## The mental model

LEVEL 1 — SYSTEM MAP

Frontend

Backend

Database

Applications and their connections, flowing left to right.

LEVEL 2 — CLICK A NODE

Backend

Expanded view

Payment Flow

Auth Flow

Webhook Flow

Other flows

Each application contains its own internal flows.

LEVEL 3 — OPEN A FLOW

Payment Flow

Internal sequence and connections

Flow documentation

How it works, important details, links to relevant code, and implementation notes.

Items to complete

A lightweight checklist of pending work, with the ability to mark items done.

## How the product should work

### 1. Applications are the top-level nodes

You start with a left-to-right graph of applications, services, databases, and external systems. Connections describe how they interact.

You can select any node to explore what's inside it, rather than cramming every internal detail into the main graph.

### 2. Each node contains its own flows

Clicking a node opens an expanded view containing its flows.

For example, a backend service could contain:

- Payment processing
- Authentication
- Webhook handling
- Retry and error handling

These flows can themselves be visualized as graphs, with their own internal components and connections.

### 3. Each flow has documentation

Every flow can have associated documentation explaining its behavior and implementation.

This might include:

- What the flow does and why it exists.
- The sequence of operations and interactions.
- Relevant APIs, code paths, and repositories.
- Important edge cases and implementation details.
- Links to related flows or external documentation.

Documentation belongs to the relevant flow, so it stays close to the thing it describes.

### 4. Each flow has items to complete

This is the lightweight tracking part.

For example, a payment flow might have:

- Understand the retry mechanism.
- Document the failure cases.
- Fix the timeout handling.
- Verify webhook behavior.

You can add items, mark them complete, and see what's left. These are not full-blown project-management tickets; they're simply a persistent list of work associated with a specific part of the system.

Items could also exist at the application level when they aren't tied to one particular flow.

### 5. Everything stays connected

The hierarchy is essentially:

System graph

Application / service node

Internal flows

Flow documentation + items to complete

Connections between applications remain visible at the system level, while internal flows, documentation, and items are explored at deeper levels.

## The distinction that matters

This is not primarily a task tracker with a graph attached.

It is a graph-first system exploration and documentation tool, with lightweight work tracking built into the things you're exploring.

The graph helps you understand the system. The flows explain how individual parts work. The documentation captures what you've learned. The items help you remember and complete the work that remains.

That's the product I'd build around.