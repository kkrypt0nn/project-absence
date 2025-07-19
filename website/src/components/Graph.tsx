import {
  addEdge,
  Controls,
  MiniMap,
  ReactFlow,
  useEdgesState,
  useNodesState,
} from "@xyflow/react";
import dagre from "dagre";
import { useCallback, useState } from "react";
import Modal from "react-modal";
import DomainModal from "./Modals/DomainNode";

function generateNodesAndEdges(node, parentId = null) {
  const nodes = [];
  const edges = [];

  // Currently the graph just supports domains
  if (node.type === "domain") {
    nodes.push({
      id: node.value,
      data: { label: node.value, type: node.type, ...node.data },
    });

    if (parentId) {
      edges.push({
        id: `${parentId}-${node.value}`,
        source: parentId,
        target: node.value,
      });
    }
  }
  node.connections.forEach((connection) => {
    const childNodesAndEdges = generateNodesAndEdges(connection, node.value);
    nodes.push(...childNodesAndEdges.nodes);
    edges.push(...childNodesAndEdges.edges);
  });
  return { nodes, edges };
}

function applyDagreLayout(nodes, edges) {
  const graph = new dagre.graphlib.Graph();
  graph.setGraph({});
  graph.setDefaultEdgeLabel(() => ({}));
  nodes.forEach((node) => {
    graph.setNode(node.id, { width: 200, height: 50 });
  });
  edges.forEach((edge) => {
    graph.setEdge(edge.source, edge.target);
  });
  dagre.layout(graph);
  const positionedNodes = nodes.map((node) => {
    const nodeWithPosition = graph.node(node.id);
    return {
      ...node,
      position: { x: nodeWithPosition.x, y: nodeWithPosition.y },
    };
  });
  return positionedNodes;
}

type GraphProps = {
  data: any;
};

Modal.setAppElement("#__docusaurus");

export default function Graph({ data }: GraphProps) {
  const { nodes: initialNodes, edges: initialEdges } =
    generateNodesAndEdges(data);

  const [nodes, _, onNodesChange] = useNodesState(
    applyDagreLayout(initialNodes, initialEdges)
  );
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);
  const [selectedNodeData, setSelectedNodeData] = useState(null);
  const [isModalOpen, setIsModalOpen] = useState(false);

  const onConnect = useCallback(
    (params) => setEdges((eds) => addEdge(params, eds)),
    [setEdges]
  );

  const onNodeClick = useCallback((_, node) => {
    setSelectedNodeData(node.data);
    setIsModalOpen(true);
  }, []);

  const closeModal = () => {
    setIsModalOpen(false);
    setSelectedNodeData(null);
  };

  const modalContentForNode = () => {
    switch (selectedNodeData.type) {
      case "domain":
        return (
          <DomainModal
            label={selectedNodeData.label}
            flags={selectedNodeData.flags}
            possible_takeover_platform={selectedNodeData.possible_takeover}
          />
        );
    }
  };

  return (
    <>
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
        onNodeClick={onNodeClick}
        colorMode="dark"
      >
        <MiniMap />
        <Controls />
      </ReactFlow>

      <Modal
        isOpen={isModalOpen}
        onRequestClose={closeModal}
        style={{
          overlay: {
            backgroundColor: "rgba(0, 0, 0, 0.75)",
            display: "flex",
            justifyContent: "center",
            alignItems: "center",
          },
          content: {
            color: "var(--ifm-font-color-base)",
            backgroundColor: "var(--ifm-modal-background-color)",
            border: "0px",
            borderRadius: "8px",
            width: "90%",
            maxWidth: "600px",
            height: "auto",
            maxHeight: "80vh",
            padding: "20px",
            position: window.innerWidth <= 678 ? "" : "relative",
            overflow: "auto",
          },
        }}
      >
        <button
          onClick={closeModal}
          className="button button--icon button--close"
          style={{
            color: "var(--ifm-font-color-base)",
            position: "absolute",
            top: "10px",
            right: "10px",
          }}
        >
          &times;
        </button>
        {selectedNodeData && (
          <>
            <h3>
              {selectedNodeData.type.charAt(0).toUpperCase() +
                selectedNodeData.type.slice(1)}{" "}
              Information
            </h3>
            {modalContentForNode()}
          </>
        )}
      </Modal>
    </>
  );
}
