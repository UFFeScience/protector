# Cover Crime Problem

This project provides tools to support **preventive police patrol planning** through optimization and heuristic techniques.
The goal is to improve the allocation and routing of police patrol units in urban environments by maximizing coverage of crime hotspots while respecting operational constraints such as route length, available vehicles, and territorial zones.

The project is inspired by research on the **Routing of Police Vehicles in Large Urban Centers (RVP-Urb)** problem and implements heuristic-based approaches to generate effective patrol routes.

---

## Problem Overview

Urban crime prevention is a major challenge for large cities. Police departments typically have **limited patrol resources** (vehicles and officers) compared to the size and complexity of the urban environment.

A common preventive strategy is **patrolling areas with higher crime probability**, often called **crime hotspots**. However, defining patrol routes manually is difficult because it must consider:

- Crime distribution across the city
- Limited number of patrol units
- Maximum allowed patrol distance
- Mandatory locations (e.g., schools, banks)
- Territorial divisions used by police operations

The **RVP-Urb problem** models the city as a **graph**, where:

- **Vertices** represent street intersections or relevant locations
- **Edges** represent street segments
- Each edge has attributes such as:
  - crime index
  - length
  - traffic direction

The objective is to **maximize the total crime index covered by patrol routes** while respecting operational constraints.

This problem is **NP-complete**, meaning that exact mathematical models become impractical for large cities. For this reason, heuristic and metaheuristic approaches are required.

---

## Solution Approach

This project focuses on heuristic-based solutions inspired by the **PROTECTOR algorithm**, which combines:

- **GRASP (Greedy Randomized Adaptive Search Procedure)**
- **Variable Neighborhood Descent (VND)** local search

The algorithm works in two main phases.

### 1. Construction Phase

An initial patrol solution is built using greedy randomized rules:

- Identify high-crime vertices
- Generate patrol routes that prioritize crime hotspots
- Create two types of routes:
  - **Inter-zone routes**: may cross multiple zones
  - **Intra-zone routes**: restricted to a specific zone
- Optionally include **mandatory edges** that must be patrolled

### 2. Local Search Phase

The initial solution is improved using neighborhood search strategies:

- **Path Switch** – replace a route segment with a higher crime coverage path
- **Loop Insertion** – extend a route to include additional high-crime areas
- **Guard Shift** – relocate fixed officers to vertices with higher uncovered crime index

The best solution found across iterations is returned as the final patrol plan.

---

## Data Model

The system represents the city using a **crime graph**:

Gc = (V, E, Q)

Where:

- **V** = vertices (street connections)
- **E** = edges (street segments)
- **Q** = partition of vertices into patrol zones

Each edge contains:

- crime index
- segment length
- direction (one-way or two-way)

Additional resources modeled in the problem include:

- **Drivable units** – police vehicles assigned to patrol routes
- **Non-drivable units** – officers assigned to fixed positions
- **Route constraints** – maximum patrol length and mandatory coverage points

---

## Dataset

Experiments described in the reference work use the **PolRoute-DS dataset**, which contains:

- ~7.2M crime records
- street segments and intersections
- geographic zones
- timestamps and crime categories

Crime categories include:

- mobile phone theft
- robbery
- vehicle theft
- vehicle robbery

These records are used to compute **crime indexes for street segments**.

---

## How to Use

Example CLI usage (conceptual):

```bash
cargo run --release -- \
  --input crime_graph.json \
  --vehicles 4 \
  --max-distance 5000 \
  --zones 3

src/
 ├─ graph/        # Crime graph data structures
 ├─ routing/      # Route construction algorithms
 ├─ heuristics/   # GRASP and local search procedures
 ├─ datasets/     # Dataset loaders and preprocessing
 └─ cli/          # Command-line interface
