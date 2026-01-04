/**
 * StateMachineDiagram component - Visual state machine diagram
 * Shows current state and valid transitions
 */

import type { AgentState } from '../../types';

interface StateMachineDiagramProps {
  currentState?: AgentState;
}

const states: AgentState[] = ['idle', 'thinking', 'toolcall', 'reflecting'];

const transitions: Array<[AgentState, AgentState]> = [
  ['idle', 'thinking'],
  ['thinking', 'toolcall'],
  ['thinking', 'reflecting'],
  ['toolcall', 'reflecting'],
  ['reflecting', 'idle'],
];

const stateLabels: Record<AgentState, string> = {
  idle: 'Idle',
  thinking: 'Thinking',
  toolcall: 'Tool Call',
  reflecting: 'Reflecting',
};

const statePositions: Record<AgentState, { x: number; y: number }> = {
  idle: { x: 50, y: 50 },
  thinking: { x: 50, y: 25 },
  toolcall: { x: 75, y: 50 },
  reflecting: { x: 50, y: 75 },
};

export function StateMachineDiagram({ currentState }: StateMachineDiagramProps) {
  return (
    <div className="card">
      <h3 className="text-h4 font-display text-cyan-electric mb-4">State Machine</h3>
      <div className="relative h-64 bg-deep-navy rounded-lg border border-cyan-electric/20 p-4">
        <svg viewBox="0 0 100 100" className="w-full h-full">
          {/* Draw transitions */}
          {transitions.map(([from, to], idx) => {
            const fromPos = statePositions[from];
            const toPos = statePositions[to];
            const isActive = currentState === from || currentState === to;
            return (
              <line
                key={`${from}-${to}-${idx}`}
                x1={fromPos.x}
                y1={fromPos.y}
                x2={toPos.x}
                y2={toPos.y}
                stroke={isActive ? '#00D9FF' : '#9E9E9E'}
                strokeWidth={isActive ? '0.5' : '0.2'}
                opacity={isActive ? '0.8' : '0.3'}
              />
            );
          })}

          {/* Draw states */}
          {states.map((state) => {
            const pos = statePositions[state];
            const isCurrent = currentState === state;
            return (
              <g key={state}>
                <circle
                  cx={pos.x}
                  cy={pos.y}
                  r="8"
                  fill={isCurrent ? '#00D9FF' : '#424242'}
                  stroke={isCurrent ? '#00D9FF' : '#9E9E9E'}
                  strokeWidth={isCurrent ? '0.5' : '0.2'}
                  className={isCurrent ? 'animate-pulse' : ''}
                />
                <text
                  x={pos.x}
                  y={pos.y - 12}
                  textAnchor="middle"
                  fontSize="3"
                  fill={isCurrent ? '#00D9FF' : '#9E9E9E'}
                  fontWeight={isCurrent ? 'bold' : 'normal'}
                >
                  {stateLabels[state]}
                </text>
              </g>
            );
          })}
        </svg>
      </div>
      {currentState && (
        <p className="text-body-sm text-medium-gray mt-4 text-center">
          Current state: <span className="text-cyan-electric font-medium capitalize">{currentState}</span>
        </p>
      )}
    </div>
  );
}

