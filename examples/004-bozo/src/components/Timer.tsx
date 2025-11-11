import { useState, useEffect } from 'react';
import { getTimeRemaining, getTimerColor } from '../lib/utils';
import { ProgressRing } from './ProgressRing';

interface TimerProps {
  deadline: string;
  size?: number;
}

export function Timer({ deadline, size = 200 }: TimerProps) {
  const [timeRemaining, setTimeRemaining] = useState(getTimeRemaining(deadline));

  useEffect(() => {
    const interval = setInterval(() => {
      setTimeRemaining(getTimeRemaining(deadline));
    }, 1000);

    return () => clearInterval(interval);
  }, [deadline]);

  const totalMinutes = 45;
  const progress = (timeRemaining.total / (totalMinutes * 60 * 1000)) * 100;

  return (
    <ProgressRing progress={progress} size={size} strokeWidth={12}>
      <div className="flex flex-col items-center">
        <span className={`font-mono tabular-nums ${getTimerColor(timeRemaining.total)}`}
          style={{ fontSize: size * 0.25, lineHeight: 1 }}>
          {timeRemaining.formatted}
        </span>
        <span className="text-muted-foreground text-xs mt-1">
          {timeRemaining.total < 60000 ? 'HURRY!' : 'remaining'}
        </span>
      </div>
    </ProgressRing>
  );
}
