import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { Button } from '../components/ui/button';
import { Card, CardContent } from '../components/ui/card';
import { Tabs, TabsList, TabsTrigger, TabsContent } from '../components/ui/tabs';
import { Avatar, AvatarFallback } from '../components/ui/avatar';
import { HowItWorksDialog } from '../components/HowItWorksDialog';
import { LeaderboardEntry } from '../types';
import { mockApi } from '../lib/mock-api';
import { ArrowLeft, Trophy, Zap, Target, HelpCircle } from 'lucide-react';

export function Stats() {
  const navigate = useNavigate();
  const [leaderboard, setLeaderboard] = useState<LeaderboardEntry[]>([]);
  const [leaderboardType, setLeaderboardType] = useState<'winners' | 'first' | 'sniped'>('winners');
  const [howItWorksOpen, setHowItWorksOpen] = useState(false);

  useEffect(() => {
    loadLeaderboard();
  }, [leaderboardType]);

  const loadLeaderboard = async () => {
    try {
      const result = await mockApi.getLeaderboard(leaderboardType);
      setLeaderboard(result);
    } catch (error) {
      console.error('Failed to load leaderboard:', error);
    }
  };

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
                  onClick={() => navigate('/stats')}
                  className="text-sm text-foreground hover:text-[#F6C445] transition-colors"
                >
                  LEADERBOARD
                </button>
                <button 
                  onClick={() => navigate('/faq')}
                  className="text-sm text-muted-foreground hover:text-foreground transition-colors"
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
      <div className="container mx-auto px-4 py-12 max-w-4xl">
        <div className="text-center mb-8">
          <h1 className="text-foreground mb-2 text-4xl tracking-wider">LEADERBOARD</h1>
          <p className="text-muted-foreground">All-time leaders</p>
        </div>

        <Tabs value={leaderboardType} onValueChange={(val) => setLeaderboardType(val as any)}>
          <TabsList className="grid w-full grid-cols-3 bg-[#252840] mb-8">
            <TabsTrigger value="winners">
              <Trophy className="w-4 h-4 mr-2" />
              Top Bozos
            </TabsTrigger>
            <TabsTrigger value="first">
              <Zap className="w-4 h-4 mr-2" />
              First Bozos
            </TabsTrigger>
            <TabsTrigger value="sniped">
              <Target className="w-4 h-4 mr-2" />
              Most Bozo&apos;d
            </TabsTrigger>
          </TabsList>

          <Card className="bg-card border-border">
            <CardContent className="p-6">
              <div className="space-y-3">
                {leaderboard.map((entry) => (
                  <div
                    key={entry.rank}
                    className="flex items-center justify-between p-4 rounded-lg bg-[#252840] hover:bg-[#252840]/80 transition-colors"
                  >
                    <div className="flex items-center gap-4">
                      <div className="w-12 text-center">
                        {entry.rank <= 3 ? (
                          <span className="text-2xl">
                            {entry.rank === 1 ? '👑' : entry.rank === 2 ? '🥈' : '🥉'}
                          </span>
                        ) : (
                          <span className="text-muted-foreground">#{entry.rank}</span>
                        )}
                      </div>
                      <Avatar className="w-12 h-12">
                        <AvatarFallback className={
                          entry.rank === 1 ? "bg-[#F6C445] text-[#0E1020]" : "bg-[#FF4B4B] text-[#FFF2E1]"
                        }>
                          {entry.handle?.[0]?.toUpperCase() || 'B'}
                        </AvatarFallback>
                      </Avatar>
                      <div>
                        <div className="text-foreground">{entry.handle || entry.address}</div>
                        <div className="text-sm text-muted-foreground">
                          {leaderboardType === 'winners' && `${entry.wins} wins`}
                          {leaderboardType === 'first' && `${entry.firstIns} first deposits`}
                          {leaderboardType === 'sniped' && `sniped ${entry.timesSniped} times`}
                        </div>
                      </div>
                    </div>
                    <div className="text-right">
                      {leaderboardType === 'winners' && (
                        <div className="text-[#2ED4B7]">${entry.totalWon?.toLocaleString()}</div>
                      )}
                      {leaderboardType === 'first' && (
                        <div className="text-[#F6C445]">${entry.totalDeposited?.toLocaleString()}</div>
                      )}
                      {leaderboardType === 'sniped' && (
                        <div className="text-[#FF4B4B]">RIP BOZO</div>
                      )}
                    </div>
                  </div>
                ))}
              </div>
            </CardContent>
          </Card>
        </Tabs>
      </div>
    </div>
  );
}
