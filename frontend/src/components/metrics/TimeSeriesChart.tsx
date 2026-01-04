/**
 * TimeSeriesChart component - Line chart for time series data
 * Uses Recharts for visualization
 */

import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts';

interface TimeSeriesDataPoint {
  time: string;
  [key: string]: string | number;
}

interface TimeSeriesChartProps {
  data: TimeSeriesDataPoint[];
  lines: Array<{ key: string; name: string; color: string }>;
  title?: string;
  height?: number;
}

export function TimeSeriesChart({ data, lines, title, height = 300 }: TimeSeriesChartProps) {
  return (
    <div className="card">
      {title && (
        <h3 className="text-h4 font-display text-cyan-electric mb-4">{title}</h3>
      )}
      <ResponsiveContainer width="100%" height={height}>
        <LineChart data={data} margin={{ top: 5, right: 20, left: 0, bottom: 5 }}>
          <CartesianGrid strokeDasharray="3 3" stroke="#00D9FF" opacity={0.2} />
          <XAxis 
            dataKey="time" 
            stroke="#9E9E9E"
            style={{ fontSize: '12px' }}
          />
          <YAxis 
            stroke="#9E9E9E"
            style={{ fontSize: '12px' }}
          />
          <Tooltip
            contentStyle={{
              backgroundColor: '#16213E',
              border: '1px solid #00D9FF',
              borderRadius: '8px',
              color: '#FFFFFF',
            }}
          />
          <Legend 
            wrapperStyle={{ color: '#E0E0E0' }}
          />
          {lines.map((line) => (
            <Line
              key={line.key}
              type="monotone"
              dataKey={line.key}
              name={line.name}
              stroke={line.color}
              strokeWidth={2}
              dot={false}
              activeDot={{ r: 4 }}
            />
          ))}
        </LineChart>
      </ResponsiveContainer>
    </div>
  );
}

