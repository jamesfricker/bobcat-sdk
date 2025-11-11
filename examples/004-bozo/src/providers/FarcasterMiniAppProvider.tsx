import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from 'react';
import {
  sdk as miniAppSdk,
  type ComposeCast,
  type MiniAppContext,
  type SetPrimaryButtonOptions,
  type SignIn,
} from '@farcaster/miniapp-sdk';

type PrimaryButtonHandler = () => void;

type FarcasterMiniAppState = {
  isMiniApp: boolean;
  ready: boolean;
  context: MiniAppContext | null;
  setPrimaryButton: (options: SetPrimaryButtonOptions) => Promise<void>;
  hidePrimaryButton: () => Promise<void>;
  onPrimaryButtonClick: (handler: PrimaryButtonHandler) => () => void;
  composeCast: (
    options?: ComposeCast.Options | string
  ) => Promise<ComposeCast.Result | undefined>;
  openUrl: (url: string | { url: string }) => Promise<void>;
  signIn: (options?: SignIn.SignInOptions) => Promise<SignIn.SignInResult | undefined>;
};

const FarcasterMiniAppContext = createContext<FarcasterMiniAppState | undefined>(undefined);

function normalizeComposeOptions(
  options?: ComposeCast.Options | string
): ComposeCast.Options | undefined {
  if (!options) {
    return undefined;
  }

  if (typeof options === 'string') {
    return { text: options };
  }

  return options;
}

export function FarcasterMiniAppProvider({ children }: { children: ReactNode }) {
  const [isMiniApp, setIsMiniApp] = useState(false);
  const [ready, setReady] = useState(false);
  const [context, setContext] = useState<MiniAppContext | null>(null);

  useEffect(() => {
    let cancelled = false;

    const initialize = async () => {
      try {
        const detected = await miniAppSdk.isInMiniApp();
        if (cancelled) {
          return;
        }

        setIsMiniApp(detected);
        if (!detected) {
          return;
        }

        try {
          const ctx = await miniAppSdk.context;
          if (!cancelled) {
            setContext(ctx);
          }
        } catch (error) {
          console.error('Failed to load Farcaster mini app context', error);
        }

        try {
          await miniAppSdk.actions.ready();
          if (!cancelled) {
            setReady(true);
          }
        } catch (error) {
          console.error('Failed to signal Farcaster mini app readiness', error);
        }
      } catch (error) {
        console.error('Failed to detect Farcaster mini app environment', error);
      }
    };

    void initialize();

    return () => {
      cancelled = true;
    };
  }, []);

  const setPrimaryButton = useCallback(
    async (options: SetPrimaryButtonOptions) => {
      if (!isMiniApp || !ready) {
        return;
      }

      try {
        await Promise.resolve(miniAppSdk.actions.setPrimaryButton(options));
      } catch (error) {
        console.error('Failed to update Farcaster primary button', error);
      }
    },
    [isMiniApp, ready]
  );

  const hidePrimaryButton = useCallback(async () => {
    if (!isMiniApp || !ready) {
      return;
    }

    try {
      await Promise.resolve(
        miniAppSdk.actions.setPrimaryButton({ text: '', hidden: true, disabled: true })
      );
    } catch (error) {
      console.error('Failed to hide Farcaster primary button', error);
    }
  }, [isMiniApp, ready]);

  const onPrimaryButtonClick = useCallback(
    (handler: PrimaryButtonHandler) => {
      if (!isMiniApp) {
        return () => {};
      }

      miniAppSdk.on('primaryButtonClicked', handler);
      return () => {
        miniAppSdk.off('primaryButtonClicked', handler);
      };
    },
    [isMiniApp]
  );

  const composeCast = useCallback(
    async (options?: ComposeCast.Options | string) => {
      if (!isMiniApp || !ready) {
        throw new Error('Farcaster mini app not detected');
      }

      const normalized = normalizeComposeOptions(options);
      try {
        return await miniAppSdk.actions.composeCast(normalized);
      } catch (error) {
        console.error('Failed to open Farcaster composer', error);
        throw error instanceof Error ? error : new Error('Compose cast failed');
      }
    },
    [isMiniApp, ready]
  );

  const openUrl = useCallback(
    async (url: string | { url: string }) => {
      if (isMiniApp) {
        try {
          await miniAppSdk.actions.openUrl(url);
          return;
        } catch (error) {
          console.error('Failed to open URL in Farcaster mini app', error);
        }
      }

      const targetUrl = typeof url === 'string' ? url : url.url;
      if (typeof window !== 'undefined') {
        window.open(targetUrl, '_blank', 'noopener,noreferrer');
      }
    },
    [isMiniApp]
  );

  const signIn = useCallback(
    async (options?: SignIn.SignInOptions) => {
      if (!isMiniApp) {
        return undefined;
      }

      try {
        const result = await miniAppSdk.actions.signIn(options);
        return result;
      } catch (error) {
        if (error instanceof SignIn.RejectedByUser) {
          return undefined;
        }
        console.error('Failed to sign in with Farcaster mini app', error);
        throw error;
      }
    },
    [isMiniApp]
  );

  const value = useMemo(
    () => ({
      isMiniApp,
      ready,
      context,
      setPrimaryButton,
      hidePrimaryButton,
      onPrimaryButtonClick,
      composeCast,
      openUrl,
      signIn,
    }),
    [
      composeCast,
      context,
      hidePrimaryButton,
      isMiniApp,
      onPrimaryButtonClick,
      openUrl,
      ready,
      setPrimaryButton,
      signIn,
    ]
  );

  return (
    <FarcasterMiniAppContext.Provider value={value}>
      {children}
    </FarcasterMiniAppContext.Provider>
  );
}

export function useFarcasterMiniApp() {
  const contextValue = useContext(FarcasterMiniAppContext);
  if (!contextValue) {
    throw new Error('useFarcasterMiniApp must be used within a FarcasterMiniAppProvider');
  }
  return contextValue;
}
