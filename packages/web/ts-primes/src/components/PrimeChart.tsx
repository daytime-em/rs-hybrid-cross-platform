import React from "react";
import { AxisOptions, Chart } from "react-charts";

export interface PrimeChartProps {
  approxPrimes: number[];
}

export type PrimeChart<Props extends PrimeChartProps> = (
  props: Props,
  context?: any
) => PrimeChart<any> | null | JSX.Element;

type Datum = {
  x: number;
  y: number;
};

/**
 * A chart of prime numbers, or their approximate density for large sets
 *
 * @param props
 * @returns
 */
export const PrimeChart = (props: PrimeChartProps) => {
  const approxPrimes = props.approxPrimes;
  const chartData = [];
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

  // const primaryAxis = React.useMemo<AxisOptions<Datum>> (
  //   () => ({
  //     getValue: (datum: Datum) => datum.x,
  //   }),
  //   []
  // );
  // const secondaryAxis = React.useMemo<AxisOptions<Datum>>(
  //   () => ({
  //     getValue: (datum: Datum) => datum.y,
  //   }),
  //   []
  // );
  const primaryAxis = React.useMemo<
    AxisOptions<typeof data[number]["data"][number]>
  >(
    () => ({
      getValue: (datum) => datum.x as unknown as number,
    }),
    []
  );

  const secondaryAxes = React.useMemo<
    AxisOptions<typeof data[number]["data"][number]>[]
  >(
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
