// Analytics event dispatcher (stub for development)

export type AnalyticsEvent =
  | { name: 'view_lobby'; props: { sort: string; filter: string } }
  | { name: 'view_game'; props: { gameId: string; status: string } }
  | { name: 'connect_wallet'; props: { wallet: string } }
  | { name: 'connect_farcaster'; props: { fid: number } }
  | { name: 'route_quote_request'; props: { gameId: string; sourceChain: string; sourceAsset: string; amountSource: string } }
  | { name: 'route_quote_response'; props: { router: string; estArrivalSec: number; meetsMinPct: boolean } }
  | { name: 'deposit_click'; props: { gameId: string; amountToken: string; localOrBridge: string } }
  | { name: 'pending_intent_created'; props: { expiresInSec: number } }
  | { name: 'deposit_committed'; props: { txHash: string } }
  | { name: 'deposit_confirmed_on_home'; props: { amountToken: string } }
  | { name: 'timer_extended_soft'; props: { sec: number } }
  | { name: 'finalize_start'; props: { gameId: string } }
  | { name: 'rng_complete'; props: { winnerAddr: string } }
  | { name: 'claim_click'; props: { type: string } }
  | { name: 'claim_success'; props: { type: string; amountToken: string } }
  | { name: 'share_click'; props: { channel: string } }
  | { name: 'error'; props: { code: string; message: string; context: string } };

export function trackEvent(event: AnalyticsEvent) {
  // In production, this would send to your analytics service
  console.log('[Analytics]', event.name, event.props);
  
  // Example integrations:
  // - PostHog: posthog.capture(event.name, event.props)
  // - Mixpanel: mixpanel.track(event.name, event.props)
  // - Google Analytics: gtag('event', event.name, event.props)
}
