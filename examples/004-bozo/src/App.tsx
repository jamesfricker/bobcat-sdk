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

export default function App() {
  useEffect(() => {
    // Force dark mode
    document.documentElement.classList.add("dark");
  }, []);

  return (
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
  );
}