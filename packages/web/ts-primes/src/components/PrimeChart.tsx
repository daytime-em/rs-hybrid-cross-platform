import React from "react";
import { AxisOptions, Chart } from "react-charts";

export interface PrimeChartProps {
  approxPrimes: number[];
}

type Datum = {
  x: number;
  y: number;
};

// export const PrimeChart = (props: PrimeChartProps) => {
/**
 * A chart of prime numbers, or their approximate density for large sets
 *
 * @param props
 * @returns
 */
export function PrimeChart(props: PrimeChartProps) {
  const approxPrimes = props.approxPrimes;
  const chartData: Datum[] = [];
  for (let idx = 0; idx < approxPrimes.length; idx++) {
    chartData[idx] = { x: idx, y: approxPrimes[idx] };
  }

  console.log("chartData data: ", chartData);

  type Series = {
    label: string;
    data: Datum[];
  };

  const data: Series[] = [
    {
      label: "Approximate # of Primes",
      data: chartData,
    },
  ];

  const primaryAxis = React.useMemo<AxisOptions<Datum>>(
    () => ({
      getValue: (datum) => datum.x,
    }),
    []
  );
  const secondaryAxes = React.useMemo<AxisOptions<Datum>[]>(
    () => [
      {
        getValue: (datum) => datum.y,
      },
    ],
    []
  );

  return (
    <Chart
      options={{
        data,
        primaryAxis,
        secondaryAxes
      }}
    />
  );
};
