# Rubl - eBird API MCP Server

An MCP (Model Context Protocol) server providing access to the eBird API for querying birding hotspots, region hierarchies, and rare bird sightings.

## Features

Rubl provides four powerful tools for accessing eBird data:

- **`get_region_info`** - Look up eBird region metadata (name, type, bounds, parent region)
- **`get_subregions`** - List subregions (states, counties, etc.) under an eBird region
- **`fetch_rare`** - Fetch recently reported notable/rare bird sightings for a region
- **`get_nearby_hotspots`** - Find eBird birding hotspots by coordinates with optional radius and date filters

## Installation

### MCPB Bundle (Recommended)

1. Download the latest `.mcpb` bundle from the [releases page](https://github.com/drewbxyz/rubl/releases)
2. Open with Claude for macOS/Windows for single-click installation
3. Configure your eBird API key when prompted

### Get an eBird API Key

Get a free eBird API key at: https://ebird.org/api/keygen

### Manual Installation

Alternatively, add to your Claude Desktop configuration (`~/Library/Application Support/Claude/claude_desktop_config.json`):

```json
{
  "mcpServers": {
    "rubl": {
      "command": "/path/to/rubl/server/rubl",
      "env": {
        "EBIRD_API_KEY": "your-api-key-here"
      }
    }
  }
}
```

## Building from Source

### Prerequisites

- Rust 1.70+ (edition 2024)
- Cargo

### Build

```bash
cargo build --release
```

The binary will be available at `target/release/rubl`.

### Build MCPB Bundle

```bash
./build-mcpb.sh
```

This creates `rubl.mcpb` ready for distribution.

## Usage

Once installed in Claude Desktop, you can use prompts like:

- "What rare birds have been seen in North Carolina recently?"
- "Find birding hotspots near coordinates 35.7796, -78.6382"
- "Show me the counties in California"
- "What are the geographic bounds of the US-NC region?"

## Tool Details

### get_region_info

Get metadata for an eBird region code (e.g., `US-NC`).

**Parameters:**
- `region_code` (string) - eBird region code

**Returns:** Region name, type, bounds, parent region, and coordinates

### get_subregions

List all subregions within a parent region.

**Parameters:**
- `region_code` (string) - eBird region code

**Returns:** Array of subregions with codes and names

### fetch_rare

Get recent notable/rare bird sightings for a region.

**Parameters:**
- `region_code` (string) - eBird region code

**Returns:** Array of rare bird observations with species, location, date, and count

### get_nearby_hotspots

Find birding hotspots near a location.

**Parameters:**
- `latitude` (number) - Latitude coordinate
- `longitude` (number) - Longitude coordinate
- `radius` (number, optional) - Search radius in kilometers (0-500)
- `back` (number, optional) - Only hotspots visited in last N days (1-30)

**Returns:** Array of hotspots with names, coordinates, and recent activity statistics

## eBird Region Codes

eBird uses hierarchical region codes:
- **Country:** `US`, `CA`, `MX`
- **State/Province:** `US-NC`, `CA-ON`
- **County:** `US-NC-067` (Wake County, NC)

Use `get_region_info` to validate codes and `get_subregions` to explore the hierarchy.

## Development

### Project Structure

```
rubl/
├── src/
│   ├── main.rs           # Entry point
│   ├── server.rs         # MCP server implementation
│   ├── ebird_api.rs      # eBird API client
│   └── tools/            # Tool implementations
│       ├── hotspot.rs
│       ├── rare_birds.rs
│       └── region.rs
├── manifest.json         # MCPB manifest
├── server/               # Bundle directory
│   └── rubl             # Compiled binary
└── Cargo.toml
```

### Testing

```bash
# Run with test API key
EBIRD_API_KEY=your-key cargo run
```

The server communicates via stdio using the MCP protocol.

## License

MIT License - see LICENSE file for details.

## Privacy & Security

**What Rubl Accesses:**
- Makes HTTP requests to the public eBird API (api.ebird.org) using your API key
- Your eBird API key is stored locally in Claude Desktop's configuration

**What Rubl Does NOT Access:**
- Your filesystem or local files
- Any system commands or processes
- Any data outside of eBird's public API

**Read-Only Tools:** All four tools (`get_region_info`, `get_subregions`, `fetch_rare`, `get_nearby_hotspots`) are read-only and only query publicly available eBird data.

**Installation Warning:** When installing the MCPB bundle, Claude Desktop may show a generic security warning. This is standard for all MCP bundles. Rubl is open source - you can review the code at https://github.com/drewbxyz/rubl to verify what it does.

## Links

- **eBird API Documentation:** https://documenter.getpostman.com/view/664302/S1ENwy59
- **MCP Protocol:** https://modelcontextprotocol.io
- **MCPB Specification:** https://github.com/anthropics/mcpb

## Support

Report issues at: https://github.com/drewbxyz/rubl/issues
