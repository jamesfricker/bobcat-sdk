import { Trophy, Ticket, Clock } from 'lucide-react';
import { useState, useEffect } from 'react';
import { Winners } from '../types';

interface EndGameScreenProps {
  winners: Winners;
  nextGameStartsAt: Date;
  homeToken: string;
}

export function EndGameScreen({ winners, nextGameStartsAt, homeToken }: EndGameScreenProps) {
  const [timeLeft, setTimeLeft] = useState(0);

  useEffect(() => {
    const updateTimer = () => {
      const now = Date.now();
      const diff = nextGameStartsAt.getTime() - now;
      setTimeLeft(Math.max(0, Math.floor(diff / 1000)));
    };

    updateTimer();
    const interval = setInterval(updateTimer, 1000);
    return () => clearInterval(interval);
  }, [nextGameStartsAt]);

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, '0')}`;
  };

  const formatTokenAmount = (amount: string, decimals: number = 2) => {
    return parseFloat(amount).toFixed(decimals);
  };

  const formatAddress = (address: string) => {
    return `${address.slice(0, 6)}...${address.slice(-4)}`;
  };

  return (
    <div className="min-h-[600px] flex items-center justify-center p-8">
      <div className="w-full max-w-3xl space-y-8">
        {/* Game Over Header */}
        <div className="text-center space-y-4">
          <div className="text-6xl mb-4">🤡</div>
          <h1 className="text-4xl text-[#FFF2E1] tracking-wide">
            GAME OVER
          </h1>
          <p className="text-xl text-[#FF4B4B] tracking-wider">
            RIP BOZO
          </p>
        </div>

        {/* Winner Section */}
        <div className="bg-[#1a1d35] border-2 border-[#F6C445] rounded-xl p-8">
          <div className="flex items-center justify-center gap-3 mb-6">
            <Trophy className="w-8 h-8 text-[#F6C445]" />
            <h2 className="text-2xl text-[#F6C445] tracking-wider">
              WINNER
            </h2>
            <Trophy className="w-8 h-8 text-[#F6C445]" />
          </div>
          
          <div className="text-center space-y-3">
            <div className="text-3xl font-mono text-[#FFF2E1]">
              {winners.winner.handle || formatAddress(winners.winner.address)}
            </div>
            <div className="text-5xl font-mono text-[#F6C445]">
              {formatTokenAmount(winners.winner.amountToken, 2)} {homeToken}
            </div>
          </div>
        </div>

        {/* Lottery Winners */}
        <div className="bg-[#1a1d35] border border-[#2a2d4a] rounded-xl p-6">
          <div className="flex items-center justify-center gap-2 mb-4">
            <Ticket className="w-5 h-5 text-[#2ED4B7]" />
            <h3 className="text-lg text-[#2ED4B7] tracking-wider">
              LOTTERY WINNERS
            </h3>
          </div>
          
          <div className="grid grid-cols-2 gap-3">
            {winners.community.map((winner, i) => (
              <div 
                key={i}
                className="flex items-center justify-between px-4 py-2 bg-[#252840]/50 rounded-lg"
              >
                <span className="text-sm font-mono text-[#FFF2E1]/80">
                  {winner.handle || formatAddress(winner.address)}
                </span>
                <span className="text-sm font-mono text-[#2ED4B7]">
                  {formatTokenAmount(winner.amountToken, 2)} {homeToken}
                </span>
              </div>
            ))}
          </div>
        </div>

        {/* Next Game Countdown */}
        <div className="bg-[#252840] border border-[#FF4B4B]/30 rounded-xl p-6">
          <div className="flex items-center justify-center gap-3 mb-3">
            <Clock className="w-5 h-5 text-[#FF4B4B]" />
            <h3 className="text-lg text-[#FFF2E1] tracking-wider">
              NEXT ROUND STARTS IN
            </h3>
          </div>
          
          <div className="text-center">
            <div className="text-5xl font-mono text-[#FF4B4B] tabular-nums">
              {formatTime(timeLeft)}
            </div>
            {timeLeft === 0 && (
              <div className="mt-3 text-[#2ED4B7] tracking-wider animate-pulse">
                STARTING NOW...
              </div>
            )}
          </div>
        </div>

        {/* Trash Talk */}
        <div className="text-center">
          <p className="text-[#FFF2E1]/60 text-lg">
            Think you can do better? 🤡
          </p>
        </div>
      </div>
    </div>
  );
}
