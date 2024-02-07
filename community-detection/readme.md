# Community Detection
Detecting Strongly Connected Components: A Deep Dive into Kosaraju's Algorithm

This is a lite version of how detecting community of trolls is done.
Based on https://neo4j.com/news/nbc-news-russian-trolls-tweets/.

## tldr; 
NBC combed through list of twitter users who paid to promote tweets close to the election period to detect Russian trolls.  

## How

Detecting a community of trolls using username as keys. Then find users who tend to mention or retweet each other.
In this simplified version, it treats the list of users as both a list of users and list of user mentioned.

ie: row 1 mention row 2. row 2 mention row 3 and onward.

## Kosaraju's algorithm

1. Create a graph and a list `L`.
2. visit each vertex of the graph `visit(n)`.
3. visit each neighbor with `visit(neighbor)`.
4. If the vertex is not visited yet, mark it as visited.
5. prepend the vertex to `L`.
6. For each vertex in `L`, assign each in-neighbor `v` to each vertex `n` in a tree manner. `Assign(vertex, root)`.

From 6, each strong community is a community of nodes that are reachable by a root.
In the end, there will be groups/components of vertices that aren't connecting to another component.