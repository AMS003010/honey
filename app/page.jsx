"use client";

import React from "react";
import RelationshipGraph from "./components/RelationshipGraph";

const data = {
  nodes: [
    { id: "Alice", group: 1 },
    { id: "Bob", group: 2 },
    { id: "Charlie", group: 1 },
    { id: "David", group: 3 },
    { id: "Eve", group: 2 },
    { id: "Frank", group: 3 },
    { id: "Grace", group: 1 },
  ],
  links: [
    { source: "Alice", target: "Bob" },
    { source: "Alice", target: "Charlie" },
    { source: "Bob", target: "David" },
    { source: "Charlie", target: "Eve" },
    { source: "Eve", target: "Frank" },
    { source: "David", target: "Grace" },
    { source: "Grace", target: "Alice" },
  ],
};


export default function Home() {
  return (
    <div>
      <h2 style={{ textAlign: "center" }}>Relationship Graph</h2>
      <RelationshipGraph data={data} />
    </div>
  );
}
