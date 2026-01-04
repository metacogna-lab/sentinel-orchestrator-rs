/**
 * BarChart component - Bar chart for categorical data
 * Uses Recharts for visualization
 */

import { BarChart as RechartsBarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts';

interface BarChartDataPoint {
  name: string;
  [key: string]: string | number;
}

interface BarChartProps {
  data: BarChartDataPoint[];
  bars: Array<{ key: string; name: string; color: string }>;
  title?: string;
  height?: number;
}

export function BarChart({ data, bars, title, height = 300 }: BarChartProps) {
  return (
    <div className="card">
      {title && (
        <h3 className="text-h4 font-display text-cyan-electric mb-4">{title}</h3>
      )}
      <ResponsiveContainer width="100%" height={height}>
        <RechartsBarChart data={data} margin={{ top: 5, right: 20, left: 0, bottom: 5 }}>
          <CartesianGrid strokeDasharray="3 3" stroke="#00D9FF" opacity={0.2} />
          <XAxis 
            dataKey="name" 
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
          {bars.map((bar) => (
            <Bar
              key={bar.key}
              dataKey={bar.key}
              name={bar.name}
              fill={bar.color}
              radius={[4, 4, 0, 0]}
            />
          ))}
        </RechartsBarChart>
      </ResponsiveContainer>
    </div>
  );
}

