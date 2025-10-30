import { useState, useEffect } from 'react';
import { Alert, AlertDescription } from './ui/alert';
import { Progress } from './ui/progress';
import { Clock, CheckCircle } from 'lucide-react';

interface PendingIntentBarProps {
  expiresAt: Date;
  softExtendSec: number;
  onExpire?: () => void;
}

export function PendingIntentBar({ expiresAt, softExtendSec, onExpire }: PendingIntentBarProps) {
  const [timeLeft, setTimeLeft] = useState(0);
  const totalTime = softExtendSec * 1000;

  useEffect(() => {
    const updateTimer = () => {
      const now = Date.now();
      const remaining = Math.max(0, expiresAt.getTime() - now);
      setTimeLeft(remaining);

      if (remaining === 0 && onExpire) {
        onExpire();
      }
    };

    updateTimer();
    const interval = setInterval(updateTimer, 1000);

    return () => clearInterval(interval);
  }, [expiresAt, onExpire]);

  if (timeLeft === 0) {
    return null;
  }

  const progress = (timeLeft / totalTime) * 100;
  const minutes = Math.floor(timeLeft / 60000);
  const seconds = Math.floor((timeLeft % 60000) / 1000);
  const formatted = `${minutes}:${seconds.toString().padStart(2, '0')}`;

  return (
    <Alert className="bg-[#2ED4B7]/10 border-[#2ED4B7]">
      <CheckCircle className="h-4 w-4 text-[#2ED4B7]" />
      <AlertDescription>
        <div className="space-y-2">
          <div className="flex items-center justify-between">
            <span className="text-foreground">
              You&apos;re reserved as next depositor
            </span>
            <div className="flex items-center gap-1 text-[#2ED4B7]">
              <Clock className="w-3 h-3" />
              <span className="font-mono">{formatted}</span>
            </div>
          </div>
          <Progress value={progress} className="h-2" />
          <p className="text-xs text-muted-foreground">
            Soft extension: +{softExtendSec / 60}:00 (expires when bridge completes or timer runs out)
          </p>
        </div>
      </AlertDescription>
    </Alert>
  );
}
