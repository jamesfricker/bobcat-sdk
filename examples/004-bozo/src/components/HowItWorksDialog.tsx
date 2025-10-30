import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription } from './ui/dialog';
import { Trophy, Clock, Users, Zap } from 'lucide-react';

interface HowItWorksDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function HowItWorksDialog({ open, onOpenChange }: HowItWorksDialogProps) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-2xl bg-[#1a1d35] border-[#2a2d4a] text-[#FFF2E1]">
        <DialogHeader>
          <div className="flex justify-center items-center gap-3 mb-6">
            <div className="w-12 h-12 rounded-full bg-[#FF4B4B] flex items-center justify-center">
              <div className="w-6 h-6 rounded-full bg-[#FFF2E1]" />
            </div>
            <span className="text-4xl tracking-wider text-[#FFF2E1]">BOZO</span>
          </div>
          <DialogTitle className="text-2xl text-center mb-4">
            How BOZO Works
          </DialogTitle>
          <DialogDescription className="sr-only">
            Learn how the BOZO game works, including timer mechanics, deposits, and winning strategies.
          </DialogDescription>
        </DialogHeader>
        
        <div className="space-y-6 py-4">
          <div className="flex gap-4">
            <div className="flex-shrink-0 w-12 h-12 rounded-full bg-[#F6C445]/10 flex items-center justify-center">
              <Clock className="w-6 h-6 text-[#F6C445]" />
            </div>
            <div>
              <h3 className="text-[#F6C445] mb-2">The Timer Game</h3>
              <p className="text-sm text-[#FFF2E1]/80 leading-relaxed">
                Each round has a 60-minute countdown timer. Every time someone makes a valid deposit (BOZO), 
                the timer resets to 60 minutes. When the timer hits zero, the round ends and the last person 
                to deposit wins!
              </p>
            </div>
          </div>

          <div className="flex gap-4">
            <div className="flex-shrink-0 w-12 h-12 rounded-full bg-[#FF4B4B]/10 flex items-center justify-center">
              <Zap className="w-6 h-6 text-[#FF4B4B]" />
            </div>
            <div>
              <h3 className="text-[#FF4B4B] mb-2">Make Your Move</h3>
              <p className="text-sm text-[#FFF2E1]/80 leading-relaxed">
                To BOZO, you must deposit at least 1% of the current pot value. You can deposit from any 
                EVM chain or asset - our backend handles cross-chain bridging automatically. Add a comment 
                to trash talk the competition!
              </p>
            </div>
          </div>

          <div className="flex gap-4">
            <div className="flex-shrink-0 w-12 h-12 rounded-full bg-[#2ED4B7]/10 flex items-center justify-center">
              <Trophy className="w-6 h-6 text-[#2ED4B7]" />
            </div>
            <div>
              <h3 className="text-[#2ED4B7] mb-2">Winner Takes Most</h3>
              <p className="text-sm text-[#FFF2E1]/80 leading-relaxed">
                The last person to deposit before the timer runs out wins 80% of the entire pot. 
                The remaining 20% is split equally among 10 randomly selected participants from that round.
              </p>
            </div>
          </div>

          <div className="flex gap-4">
            <div className="flex-shrink-0 w-12 h-12 rounded-full bg-[#FF4B4B]/10 flex items-center justify-center">
              <Users className="w-6 h-6 text-[#FF4B4B]" />
            </div>
            <div>
              <h3 className="text-[#FF4B4B] mb-2">RIP BOZO Theme</h3>
              <p className="text-sm text-[#FFF2E1]/80 leading-relaxed">
                This is a game of chicken and timing. Will you snipe the pot at the last second? 
                Or will someone else BOZO you? Every deposit is a taunt, every timer reset is a power move. 
                Only one can win. RIP BOZO! 🤡
              </p>
            </div>
          </div>
        </div>

        <div className="mt-4 p-4 bg-[#FF4B4B]/10 border border-[#FF4B4B]/20 rounded-lg">
          <p className="text-xs text-[#FFF2E1]/70 text-center">
            ⚠️ This is a high-risk game. Only deposit what you can afford to lose. 
            Past performance does not guarantee future results.
          </p>
        </div>
      </DialogContent>
    </Dialog>
  );
}
