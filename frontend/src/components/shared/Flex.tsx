import React from 'react';

// Generic Flexbox Wrapper
export interface FlexProps {
  children?: React.ReactNode;
  direction?: 'row' | 'row-reverse' | 'column' | 'column-reverse';
  wrap?: 'nowrap' | 'wrap' | 'wrap-reverse' | boolean;
  justify?: 'flex-start' | 'flex-end' | 'center' | 'space-between' | 'space-around' | 'space-evenly';
  alignItems?: 'stretch' | 'flex-start' | 'flex-end' | 'center' | 'baseline';
  alignContent?: 'flex-start' | 'flex-end' | 'center' | 'space-between' | 'space-around' | 'space-evenly' | 'stretch';
  grow?: boolean | number;
  shrink?: boolean | number;
  basis?: number | string | 'auto';
  textAlign?: 'left' | 'right' | 'center' | 'justify' | 'initial' | 'inherit';
  width?: string;
  height?: string;
  style?: React.CSSProperties;
}

function parseBoolean<T>(value: boolean | T, defaultFalse: T, defaultTrue: T): T {
  if (value === false) {
    return defaultFalse;
  }
  if (value === true) {
    return defaultTrue;
  }
  return value;
}

export const Flex = ({
  children,
  direction,
  wrap,
  justify,
  alignItems,
  alignContent,
  grow,
  shrink,
  basis,
  textAlign,
  width,
  height,
  style,
}: FlexProps) => {
  const computedStyle: React.CSSProperties = {
    display: 'flex',
    flexDirection: direction,
    flexWrap: parseBoolean(wrap, undefined, 'wrap'),
    justifyContent: justify,
    alignItems,
    alignContent,
    flexGrow: parseBoolean(grow, undefined, 1),
    flexShrink: parseBoolean(shrink, 0, undefined),
    flexBasis: basis,
    textAlign,
    width,
    height,
    ...style,
  };

  return <div style={computedStyle}>{children}</div>;
};
