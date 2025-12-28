import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Button } from '../components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '../components/ui/card';
import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from '../components/ui/accordion';
import { HowItWorksDialog } from '../components/HowItWorksDialog';
import { HelpCircle } from 'lucide-react';

export function FAQ() {
  const navigate = useNavigate();
  const [howItWorksOpen, setHowItWorksOpen] = useState(false);

  return (
    <div className="min-h-screen bg-background">
      {/* Header */}
      <header className="border-b border-border/50 bg-background/80 backdrop-blur-sm">
        <div className="container mx-auto px-4 py-4">
          <div className="flex items-center justify-between max-w-6xl mx-auto">
            <div className="flex items-center gap-6">
              <div className="flex items-center gap-2">
                <div className="w-8 h-8 rounded-full bg-[#FF4B4B] flex items-center justify-center">
                  <div className="w-4 h-4 rounded-full bg-[#FFF2E1]" />
                </div>
                <span className="text-foreground tracking-wider">BOZO</span>
              </div>
              <nav className="flex items-center gap-6">
                <button
                  onClick={() => navigate('/')}
                  className="text-sm text-muted-foreground hover:text-foreground transition-colors"
                >
                  GAME
                </button>
                <button
                  type="button"
                  disabled
                  className="text-sm text-muted-foreground cursor-not-allowed"
                >
                  LEADERBOARD
                </button>
                <button
                  onClick={() => navigate('/faq')}
                  className="text-sm text-foreground hover:text-[#F6C445] transition-colors"
                >
                  FAQ
                </button>
                <button
                  onClick={() => setHowItWorksOpen(true)}
                  className="text-sm text-muted-foreground hover:text-foreground transition-colors flex items-center gap-1"
                >
                  <HelpCircle className="w-4 h-4" />
                  HOW IT WORKS
                </button>
              </nav>
            </div>

            <div className="w-32" />
          </div>
        </div>
      </header>

      <HowItWorksDialog open={howItWorksOpen} onOpenChange={setHowItWorksOpen} />

      {/* Content */}
      <div className="container mx-auto px-4 py-12 max-w-3xl">
        <div className="text-center mb-12">
          <div className="flex justify-center items-center gap-4 mb-6">
            <div className="w-16 h-16 rounded-full bg-[#FF4B4B] flex items-center justify-center">
              <div className="w-8 h-8 rounded-full bg-[#FFF2E1]" />
            </div>
            <span className="text-5xl tracking-wider text-[#FFF2E1]">BOZO</span>
          </div>
          <h1 className="text-foreground mb-3">How Bozo Works</h1>
          <p className="text-muted-foreground">
            Everything you need to know about the game
          </p>
        </div>

        <Card className="bg-card border-border">
          <CardHeader>
            <CardTitle className="text-foreground">Frequently Asked Questions</CardTitle>
          </CardHeader>
          <CardContent>
            <Accordion type="single" collapsible className="w-full">
              <AccordionItem value="item-1">
                <AccordionTrigger>What is Bozo?</AccordionTrigger>
                <AccordionContent className="text-muted-foreground">
                  Bozo is a social crypto game where the last person to deposit before the timer runs out wins 80% of the pot.
                  The remaining 20% is split evenly among 10 randomly selected participants. It&apos;s simple, fun, and built for the Farcaster community.
                </AccordionContent>
              </AccordionItem>

              <AccordionItem value="item-2">
                <AccordionTrigger>How does the timer work?</AccordionTrigger>
                <AccordionContent className="text-muted-foreground">
                  Each game starts with a 45-minute countdown. Every valid deposit resets the timer back to 45 minutes.
                  When the timer reaches 0, the game ends and the last depositor wins. In the final 2 minutes, valid deposits add an extra 40 minutes to the clock to prevent last-second sniping.
                </AccordionContent>
              </AccordionItem>

              <AccordionItem value="item-3">
                <AccordionTrigger>What&apos;s the minimum deposit?</AccordionTrigger>
                <AccordionContent className="text-muted-foreground">
                  Every deposit must be at least 110% of the current pot value (in ARB). This ensures meaningful participation and prevents spam deposits.
                  The required minimum is clearly displayed in the game room.
                </AccordionContent>
              </AccordionItem>

              <AccordionItem value="item-4">
                <AccordionTrigger>Can I deposit from any chain?</AccordionTrigger>
                <AccordionContent className="text-muted-foreground">
                  Bozo now lives exclusively on Arbitrum. Deposits must be made directly on Arbitrum using the token held in the pot.
                  Make sure your wallet is connected to Arbitrum and that you&apos;ve approved the Bozo contract before you send it.
                </AccordionContent>
              </AccordionItem>

              <AccordionItem value="item-5">
                <AccordionTrigger>How are winners chosen?</AccordionTrigger>
                <AccordionContent className="text-muted-foreground">
                  When the timer hits 0, the last depositor automatically wins 80% of the pot. The 10 community winners are randomly selected using a random function
                  from all participants in that game. Each participant address can be selected multiple times, so your odds increase with more unique addresses playing.
                </AccordionContent>
              </AccordionItem>

              <AccordionItem value="item-6">
                <AccordionTrigger>How do I claim my winnings?</AccordionTrigger>
                <AccordionContent className="text-muted-foreground">
                  If you win, the rewards will be sent directly to your wallet!
                </AccordionContent>
              </AccordionItem>

              <AccordionItem value="item-7">
                <AccordionTrigger>What happens if my transaction is pending when the timer runs out?</AccordionTrigger>
                <AccordionContent className="text-muted-foreground">
                  Transactions are only counted once they confirm on Arbitrum. If your transaction is still pending when the timer expires,
                  it won&apos;t make you the winner, so consider using a higher gas price when the clock is low.
                </AccordionContent>
              </AccordionItem>

              <AccordionItem value="item-8">
                <AccordionTrigger>Is there a strategy to win?</AccordionTrigger>
                <AccordionContent className="text-muted-foreground">
                  The best strategy is to deposit when you think others won&apos;t! But remember: every deposit resets the timer, so timing is everything.
                  Depositing more than the minimum doesn&apos;t increase your chances—it just makes the pot bigger. The game is designed to be unpredictable and fun, not optimizable.
                </AccordionContent>
              </AccordionItem>

              <AccordionItem value="item-9">
                <AccordionTrigger>What does &quot;RIP Bozo&quot; mean?</AccordionTrigger>
                <AccordionContent className="text-muted-foreground">
                  &quot;RIP Bozo&quot; is the playful phrase we use when someone wins (or loses, depending on your perspective!).
                  It&apos;s a lighthearted meme that captures the spirit of the game—everyone&apos;s a clown until the timer runs out. 🤡
                </AccordionContent>
              </AccordionItem>

              <AccordionItem value="item-10">
                <AccordionTrigger>Is Bozo safe?</AccordionTrigger>
                <AccordionContent className="text-muted-foreground">
                  Bozo is not audited, and should be treated with caution. With any crypto game, only deposit what you can afford to lose. This is meant to be fun!
                </AccordionContent>
              </AccordionItem>
            </Accordion>
          </CardContent>
        </Card>

        <div className="mt-8 text-center">
          <Button
            onClick={() => navigate('/')}
            className="bg-[#FF4B4B] hover:bg-[#FF4B4B]/90 text-[#FFF2E1]"
            size="lg"
          >
            Start Playing
          </Button>
        </div>
      </div>
    </div>
  );
}
