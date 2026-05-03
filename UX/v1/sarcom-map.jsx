import { useState, useEffect, useRef } from "react";

const MOCK_RELAY = {
  id: "relay-1",
  lat: 50.9505,
  lon: 5.3525,
  label: "RELAY-1",
};

const MOCK_GATEWAY = {
  id: "gw-1",
  lat: 50.9485,
  lon: 5.3490,
  label: "GATEWAY",
};

function generateMockTrack() {
  const base = { lat: 50.9510, lon: 5.3540 };
  const points = [];
  const now = Date.now();
  for (let i = 0; i < 12; i++) {
    points.push({
      lat: base.lat + (Math.random() - 0.3) * 0.003 * i,
      lon: base.lon + (Math.random() - 0.5) * 0.002 * i,
      time: now - (12 - i) * 300000,
      seq: 30 + i,
      gps_valid: i !== 5,
      sos: i >= 10,
      battery_low: i >= 11,
    });
  }
  return points;
}

function formatAge(ms) {
  const sec = Math.floor(ms / 1000);
  if (sec < 60) return `${sec}s ago`;
  const min = Math.floor(sec / 60);
  if (min < 60) return `${min}m ago`;
  const hr = Math.floor(min / 60);
  return `${hr}h ${min % 60}m ago`;
}

function formatTime(ts) {
  const d = new Date(ts);
  return d.toLocaleTimeString("en-GB", { hour: "2-digit", minute: "2-digit", second: "2-digit" });
}

function staleness(ms) {
  if (ms < 360000) return "fresh";
  if (ms < 1200000) return "aging";
  return "stale";
}

// Inline SVG map since we can't use Leaflet tiles in artifact
// We'll build a simple canvas-based map renderer
function latLonToXY(lat, lon, bounds, width, height) {
  const x = ((lon - bounds.minLon) / (bounds.maxLon - bounds.minLon)) * width;
  const y = ((bounds.maxLat - lat) / (bounds.maxLat - bounds.minLat)) * height;
  return { x, y };
}

function MapCanvas({ tags, relay, gateway, selectedTag, onSelectTag, width, height }) {
  const canvasRef = useRef(null);
  const allPoints = [];

  Object.values(tags).forEach(t => {
    t.track.forEach(p => {
      if (p.gps_valid) allPoints.push(p);
    });
  });
  allPoints.push({ lat: relay.lat, lon: relay.lon });
  allPoints.push({ lat: gateway.lat, lon: gateway.lon });

  const padding = 0.002;
  const bounds = {
    minLat: Math.min(...allPoints.map(p => p.lat)) - padding,
    maxLat: Math.max(...allPoints.map(p => p.lat)) + padding,
    minLon: Math.min(...allPoints.map(p => p.lon)) - padding,
    maxLon: Math.max(...allPoints.map(p => p.lon)) + padding,
  };

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    const dpr = window.devicePixelRatio || 1;
    canvas.width = width * dpr;
    canvas.height = height * dpr;
    ctx.scale(dpr, dpr);

    // Background
    ctx.fillStyle = "#0a0f14";
    ctx.fillRect(0, 0, width, height);

    // Grid
    ctx.strokeStyle = "#1a2530";
    ctx.lineWidth = 0.5;
    for (let i = 0; i < 20; i++) {
      const x = (width / 20) * i;
      const y = (height / 20) * i;
      ctx.beginPath(); ctx.moveTo(x, 0); ctx.lineTo(x, height); ctx.stroke();
      ctx.beginPath(); ctx.moveTo(0, y); ctx.lineTo(width, y); ctx.stroke();
    }

    const toXY = (lat, lon) => latLonToXY(lat, lon, bounds, width, height);

    // Draw tracks
    Object.entries(tags).forEach(([tagId, tag]) => {
      const validPoints = tag.track.filter(p => p.gps_valid);
      if (validPoints.length < 2) return;

      ctx.beginPath();
      ctx.strokeStyle = tag.sos ? "rgba(255,60,60,0.5)" : "rgba(50,180,255,0.3)";
      ctx.lineWidth = 2;
      const first = toXY(validPoints[0].lat, validPoints[0].lon);
      ctx.moveTo(first.x, first.y);
      validPoints.slice(1).forEach(p => {
        const pt = toXY(p.lat, p.lon);
        ctx.lineTo(pt.x, pt.y);
      });
      ctx.stroke();

      // Track dots
      validPoints.forEach((p, i) => {
        const pt = toXY(p.lat, p.lon);
        ctx.beginPath();
        const isLatest = i === validPoints.length - 1;
        const radius = isLatest ? 7 : 3;
        ctx.arc(pt.x, pt.y, radius, 0, Math.PI * 2);

        if (p.sos) {
          ctx.fillStyle = "#ff3c3c";
        } else {
          ctx.fillStyle = isLatest ? "#32b4ff" : "rgba(50,180,255,0.5)";
        }
        ctx.fill();

        if (isLatest) {
          // Pulse ring
          ctx.beginPath();
          ctx.arc(pt.x, pt.y, 12, 0, Math.PI * 2);
          ctx.strokeStyle = p.sos ? "rgba(255,60,60,0.4)" : "rgba(50,180,255,0.3)";
          ctx.lineWidth = 1.5;
          ctx.stroke();

          // Label
          ctx.fillStyle = "#e0e8f0";
          ctx.font = "bold 11px monospace";
          ctx.fillText(`TAG-${tagId}`, pt.x + 16, pt.y - 4);
          ctx.fillStyle = "#7a8a9a";
          ctx.font = "10px monospace";
          ctx.fillText(`seq ${p.seq}`, pt.x + 16, pt.y + 10);
        }
      });
    });

    // Relay marker
    const rp = toXY(relay.lat, relay.lon);
    ctx.beginPath();
    // Diamond shape
    ctx.moveTo(rp.x, rp.y - 8);
    ctx.lineTo(rp.x + 6, rp.y);
    ctx.lineTo(rp.x, rp.y + 8);
    ctx.lineTo(rp.x - 6, rp.y);
    ctx.closePath();
    ctx.fillStyle = "#f0a030";
    ctx.fill();
    ctx.strokeStyle = "#0a0f14";
    ctx.lineWidth = 1;
    ctx.stroke();
    ctx.fillStyle = "#f0a030";
    ctx.font = "bold 10px monospace";
    ctx.fillText(relay.label, rp.x + 12, rp.y + 4);

    // Gateway marker
    const gp = toXY(gateway.lat, gateway.lon);
    ctx.beginPath();
    ctx.rect(gp.x - 6, gp.y - 6, 12, 12);
    ctx.fillStyle = "#40d080";
    ctx.fill();
    ctx.strokeStyle = "#0a0f14";
    ctx.lineWidth = 1;
    ctx.stroke();
    ctx.fillStyle = "#40d080";
    ctx.font = "bold 10px monospace";
    ctx.fillText(gateway.label, gp.x + 12, gp.y + 4);

    // Compass
    ctx.fillStyle = "#3a4a5a";
    ctx.font = "bold 12px monospace";
    ctx.fillText("N", width - 20, 18);
    ctx.beginPath();
    ctx.moveTo(width - 16, 22);
    ctx.lineTo(width - 13, 30);
    ctx.lineTo(width - 19, 30);
    ctx.closePath();
    ctx.fillStyle = "#3a4a5a";
    ctx.fill();

  }, [tags, width, height]);

  return (
    <canvas
      ref={canvasRef}
      style={{ width, height, display: "block" }}
    />
  );
}

export default function SARCOMDisplay() {
  const [tags, setTags] = useState({});
  const [now, setNow] = useState(Date.now());

  useEffect(() => {
    const track = generateMockTrack();
    const latest = track[track.length - 1];
    setTags({
      1: {
        tag_id: 1,
        track,
        latest,
        sos: latest.sos,
        battery_low: latest.battery_low,
      },
    });

    const interval = setInterval(() => setNow(Date.now()), 1000);
    return () => clearInterval(interval);
  }, []);

  const mapWidth = 540;
  const mapHeight = 430;

  const tag = tags[1];
  const lastValid = tag?.track?.filter(p => p.gps_valid).slice(-1)[0];
  const lastSighting = tag?.latest;
  const age = lastSighting ? now - lastSighting.time : null;
  const staleClass = age ? staleness(age) : "stale";

  return (
    <div style={{
      width: "100vw",
      height: "100vh",
      background: "#070b10",
      color: "#c0ccd8",
      fontFamily: "'JetBrains Mono', 'Fira Code', 'Courier New', monospace",
      fontSize: "11px",
      display: "flex",
      flexDirection: "row",
      overflow: "hidden",
      userSelect: "none",
    }}>
      {/* Map area */}
      <div style={{ flex: 1, position: "relative" }}>
        {/* Header bar */}
        <div style={{
          position: "absolute",
          top: 0,
          left: 0,
          right: 0,
          height: 32,
          background: "linear-gradient(180deg, rgba(10,15,20,0.95), rgba(10,15,20,0.7))",
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          padding: "0 12px",
          zIndex: 10,
          borderBottom: "1px solid #1a2530",
        }}>
          <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
            <span style={{ color: "#f0a030", fontWeight: "bold", fontSize: 13, letterSpacing: 2 }}>SARCOM</span>
            <span style={{ color: "#3a4a5a" }}>|</span>
            <span style={{ color: "#5a6a7a", fontSize: 10 }}>SEARCH & RESCUE COMMON OPERATING MAP</span>
          </div>
          <div style={{ display: "flex", alignItems: "center", gap: 12 }}>
            <span style={{
              width: 6, height: 6, borderRadius: "50%",
              background: "#40d080",
              display: "inline-block",
              boxShadow: "0 0 6px #40d080",
            }} />
            <span style={{ color: "#5a6a7a", fontSize: 10 }}>GW ONLINE</span>
            <span style={{ color: "#3a4a5a", fontSize: 10 }}>
              {new Date(now).toLocaleTimeString("en-GB")}
            </span>
          </div>
        </div>

        <div style={{ paddingTop: 32 }}>
          <MapCanvas
            tags={tags}
            relay={MOCK_RELAY}
            gateway={MOCK_GATEWAY}
            width={mapWidth}
            height={mapHeight}
          />
        </div>

        {/* Legend */}
        <div style={{
          position: "absolute",
          bottom: 8,
          left: 8,
          display: "flex",
          gap: 16,
          fontSize: 9,
          color: "#4a5a6a",
        }}>
          <div style={{ display: "flex", alignItems: "center", gap: 4 }}>
            <span style={{ width: 8, height: 8, borderRadius: "50%", background: "#32b4ff", display: "inline-block" }} />
            TAG
          </div>
          <div style={{ display: "flex", alignItems: "center", gap: 4 }}>
            <span style={{ width: 8, height: 8, borderRadius: "50%", background: "#ff3c3c", display: "inline-block" }} />
            SOS
          </div>
          <div style={{ display: "flex", alignItems: "center", gap: 4 }}>
            <span style={{ width: 8, height: 8, background: "#f0a030", transform: "rotate(45deg)", display: "inline-block" }} />
            RELAY
          </div>
          <div style={{ display: "flex", alignItems: "center", gap: 4 }}>
            <span style={{ width: 8, height: 8, background: "#40d080", display: "inline-block" }} />
            GATEWAY
          </div>
        </div>
      </div>

      {/* Sidebar */}
      <div style={{
        width: 240,
        borderLeft: "1px solid #1a2530",
        display: "flex",
        flexDirection: "column",
        background: "#0c1118",
        overflow: "auto",
      }}>
        {/* Tag status panel */}
        <div style={{
          padding: "12px 10px",
          borderBottom: "1px solid #1a2530",
        }}>
          <div style={{
            fontSize: 10,
            color: "#5a6a7a",
            marginBottom: 6,
            letterSpacing: 1,
          }}>ACTIVE TAGS</div>

          {tag && (
            <div style={{
              background: tag.sos ? "rgba(255,60,60,0.08)" : "rgba(50,180,255,0.05)",
              border: `1px solid ${tag.sos ? "rgba(255,60,60,0.3)" : "#1a2530"}`,
              borderRadius: 4,
              padding: "8px 10px",
            }}>
              <div style={{
                display: "flex",
                justifyContent: "space-between",
                alignItems: "center",
                marginBottom: 6,
              }}>
                <span style={{
                  fontWeight: "bold",
                  fontSize: 13,
                  color: tag.sos ? "#ff3c3c" : "#e0e8f0",
                }}>
                  TAG-1
                </span>
                {tag.sos && (
                  <span style={{
                    background: "#ff3c3c",
                    color: "#fff",
                    fontSize: 9,
                    fontWeight: "bold",
                    padding: "2px 6px",
                    borderRadius: 2,
                    letterSpacing: 1,
                    animation: "none",
                  }}>SOS</span>
                )}
              </div>

              <div style={{ display: "flex", flexDirection: "column", gap: 4 }}>
                <Row label="LAST SEEN" value={lastSighting ? formatTime(lastSighting.time) : "—"} />
                <Row
                  label="AGE"
                  value={age ? formatAge(age) : "—"}
                  valueColor={
                    staleClass === "fresh" ? "#40d080" :
                    staleClass === "aging" ? "#f0a030" : "#ff3c3c"
                  }
                />
                <Row label="SEQ" value={lastSighting ? `#${lastSighting.seq}` : "—"} />
                <Row label="FIX" value={lastSighting?.gps_valid ? "YES" : "NO FIX"} valueColor={lastSighting?.gps_valid ? "#40d080" : "#f0a030"} />
                {lastValid && (
                  <>
                    <Row label="LAT" value={lastValid.lat.toFixed(6) + "°"} />
                    <Row label="LON" value={lastValid.lon.toFixed(6) + "°"} />
                  </>
                )}
                <Row
                  label="BATT"
                  value={tag.battery_low ? "LOW" : "OK"}
                  valueColor={tag.battery_low ? "#ff3c3c" : "#40d080"}
                />
              </div>
            </div>
          )}
        </div>

        {/* Relay status */}
        <div style={{
          padding: "12px 10px",
          borderBottom: "1px solid #1a2530",
        }}>
          <div style={{
            fontSize: 10,
            color: "#5a6a7a",
            marginBottom: 6,
            letterSpacing: 1,
          }}>RELAY NODES</div>

          <div style={{
            background: "rgba(240,160,48,0.05)",
            border: "1px solid #1a2530",
            borderRadius: 4,
            padding: "8px 10px",
          }}>
            <div style={{ fontWeight: "bold", color: "#f0a030", marginBottom: 4 }}>
              RELAY-1
            </div>
            <Row label="LAT" value={MOCK_RELAY.lat.toFixed(6) + "°"} />
            <Row label="LON" value={MOCK_RELAY.lon.toFixed(6) + "°"} />
            <Row label="STATUS" value="ACTIVE" valueColor="#40d080" />
          </div>
        </div>

        {/* Gateway status */}
        <div style={{
          padding: "12px 10px",
          borderBottom: "1px solid #1a2530",
        }}>
          <div style={{
            fontSize: 10,
            color: "#5a6a7a",
            marginBottom: 6,
            letterSpacing: 1,
          }}>GATEWAY</div>

          <div style={{
            background: "rgba(64,208,128,0.05)",
            border: "1px solid #1a2530",
            borderRadius: 4,
            padding: "8px 10px",
          }}>
            <Row label="ID" value="GW-1" />
            <Row label="STATUS" value="ONLINE" valueColor="#40d080" />
            <Row label="LAST RX" value="3s ago" valueColor="#40d080" />
          </div>
        </div>

        {/* Sighting log */}
        <div style={{
          padding: "12px 10px",
          flex: 1,
          overflow: "auto",
        }}>
          <div style={{
            fontSize: 10,
            color: "#5a6a7a",
            marginBottom: 6,
            letterSpacing: 1,
          }}>SIGHTING LOG</div>

          <div style={{ display: "flex", flexDirection: "column", gap: 2 }}>
            {tag?.track?.slice().reverse().slice(0, 8).map((p, i) => (
              <div key={i} style={{
                fontSize: 9,
                color: p.sos ? "#ff3c3c" : "#5a6a7a",
                fontFamily: "monospace",
                lineHeight: 1.5,
              }}>
                {formatTime(p.time)} seq={p.seq} {p.gps_valid ? "FIX" : "NOFIX"} {p.sos ? "SOS" : ""}
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}

function Row({ label, value, valueColor }) {
  return (
    <div style={{
      display: "flex",
      justifyContent: "space-between",
      fontSize: 10,
      lineHeight: 1.6,
    }}>
      <span style={{ color: "#4a5a6a" }}>{label}</span>
      <span style={{ color: valueColor || "#c0ccd8", fontWeight: valueColor ? "bold" : "normal" }}>{value}</span>
    </div>
  );
}
