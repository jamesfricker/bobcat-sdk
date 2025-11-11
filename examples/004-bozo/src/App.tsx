import {
  BrowserRouter as Router,
  Routes,
  Route,
  Navigate,
} from "react-router-dom";
import { Game } from "./pages/Game";
import { Stats } from "./pages/Stats";
import { FAQ } from "./pages/FAQ";
import { Toaster } from "./components/ui/sonner";
import "./styles/globals.css";
import { useEffect } from "react";

import { WagmiProvider, http, createConfig } from 'wagmi';
import { arbitrum } from 'wagmi/chains';
import { injected, metaMask, walletConnect } from 'wagmi/connectors';

import { RainbowKitProvider } from '@rainbow-me/rainbowkit';

import '@rainbow-me/rainbowkit/styles.css'

import {
  QueryClient,
  QueryClientProvider,
} from '@tanstack/react-query';
import { CommentsProvider } from './providers/CommentsProvider';
import { FarcasterMiniAppProvider } from './providers/FarcasterMiniAppProvider';

const queryClient = new QueryClient();

const config = createConfig({
  chains: [arbitrum],
  connectors: [
    injected(),
    metaMask(),
  ],
  transports: {
    [arbitrum.id]: http(),
  },
})


export default function App() {
  useEffect(() => {
    // Force dark mode
    document.documentElement.classList.add("dark");
  }, []);

  return (
    <WagmiProvider config={config}>
      <QueryClientProvider client={queryClient}>
        <RainbowKitProvider>
          <FarcasterMiniAppProvider>
            <CommentsProvider>
              <div className="dark">
                <Router>
                  <Routes>
                    <Route path="/" element={<Game />} />
                    <Route path="/stats" element={<Stats />} />
                    <Route path="/faq" element={<FAQ />} />
                    {/* Catch-all route */}
                    <Route
                      path="*"
                      element={<Navigate to="/" replace />}
                    />
                  </Routes>
                  <Toaster position="bottom-right" />
                </Router>
              </div>
            </CommentsProvider>
          </FarcasterMiniAppProvider>
        </RainbowKitProvider>
      </QueryClientProvider>
    </WagmiProvider>
  );
}
