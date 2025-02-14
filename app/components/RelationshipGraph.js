import React, { useEffect, useRef } from "react";
import * as d3 from "d3";

const RelationshipGraph = ({ data }) => {
  const svgRef = useRef(null);

  useEffect(() => {
    const width = 800;
    const height = 600;

    const svg = d3
      .select(svgRef.current)
      .attr("width", "100%")
      .attr("height", "100%")
      .attr("viewBox", [-width / 2, -height / 2, width, height])
      .attr("style", "border: 1px solid black;")
      .call(
        d3
          .zoom()
          .scaleExtent([0.5, 3]) // Zoom range (min, max)
          .on("zoom", (event) => {
            g.attr("transform", event.transform);
          })
      );

    const g = svg.append("g"); // Group for zoom & pan

    const simulation = d3
      .forceSimulation(data.nodes)
      .force("link", d3.forceLink(data.links).id((d) => d.id).distance(120))
      .force("charge", d3.forceManyBody().strength(-300))
      .force("center", d3.forceCenter(0, 0));

    const link = g
      .append("g")
      .attr("stroke", "#aaa")
      .selectAll("line")
      .data(data.links)
      .enter()
      .append("line")
      .attr("stroke-width", 2);

    const node = g
      .append("g")
      .selectAll("circle")
      .data(data.nodes)
      .enter()
      .append("circle")
      .attr("r", 8)
      .attr("fill", (d) => d3.schemeCategory10[d.group % 10])
      .call(
        d3
          .drag()
          .on("start", (event, d) => {
            if (!event.active) simulation.alphaTarget(0.3).restart();
            d.fx = d.x;
            d.fy = d.y;
          })
          .on("drag", (event, d) => {
            d.fx = event.x;
            d.fy = event.y;
          })
          .on("end", (event, d) => {
            if (!event.active) simulation.alphaTarget(0);
            d.fx = null;
            d.fy = null;
          })
      );

    const labels = g
      .append("g")
      .selectAll("text")
      .data(data.nodes)
      .enter()
      .append("text")
      .text((d) => d.id)
      .attr("font-size", "12px")
      .attr("text-anchor", "middle")
      .attr("dy", -12);

    simulation.on("tick", () => {
      link
        .attr("x1", (d) => d.source.x)
        .attr("y1", (d) => d.source.y)
        .attr("x2", (d) => d.target.x)
        .attr("y2", (d) => d.target.y);

      node.attr("cx", (d) => d.x).attr("cy", (d) => d.y);

      labels.attr("x", (d) => d.x).attr("y", (d) => d.y);
    });
  }, [data]);

  return <svg ref={svgRef}></svg>;
};

export default RelationshipGraph;
