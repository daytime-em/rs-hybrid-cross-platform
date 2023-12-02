import { Chart } from 'react-charts';
import React from 'react';

/**
 * 
 * @param { approxPrimes: number[] } params 
 */
export function PrimeChart(props) {
  const approxPrimes = props.approxPrimes;
  const chartData = [];
  for (let idx = 0; idx < approxPrimes.length; idx++) {
    chartData[idx] = { x: idx, y: approxPrimes[idx] }
  }

  const primaryAxis = React.useMemo(
    () => ({
      getValue: datum => datum.x
    }),
    []
  );
  const secondaryAxis = React.useMemo(
    () => ({
      getValue: datum => datum.y
    }),
    []
  );

  return (
    <Chart
      options={{
        chartData,
        primaryAxis,
        secondaryAxis
      }}
    />
  );
}
