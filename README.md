# ClusterPOI

A command-line tool for clustering geographic points using the DBSCAN algorithm. Processes CSV files containing latitude/longitude data and outputs cluster assignments.

## Synopsis

```
clusterpoi -i <input.csv> -o <output.csv> [OPTIONS]
```

## Description

ClusterPOI uses the DBSCAN (Density-Based Spatial Clustering of Applications with Noise) algorithm to identify clusters of geographic points based on their proximity. The tool automatically detects latitude and longitude columns in your CSV file and adds cluster assignments to the output.

## Installation

### Prerequisites
- Rust (latest stable version)

### Build from Source
```bash
git clone <repository-url>
cd clusterpoi
cargo build --release
```

The binary will be available at `target/release/clusterpoi`.

## Command Line Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--input` | `-i` | Input CSV file with lat/long data | Required |
| `--output` | `-o` | Output CSV file with cluster information | Required |
| `--epsilon` | | Maximum distance between points in a cluster (kilometers) | 1.0 |
| `--min-samples` | | Minimum number of points required to form a cluster | 5 |

## Usage Examples

### Basic Usage
```bash
clusterpoi -i locations.csv -o clustered_locations.csv
```

### Custom Parameters
```bash
# Tighter clustering (0.5km radius, minimum 3 points)
clusterpoi -i data.csv -o results.csv --epsilon 0.5 --min-samples 3

# Looser clustering (2km radius, minimum 10 points)
clusterpoi -i data.csv -o results.csv --epsilon 2.0 --min-samples 10
```

## Input Format

The input CSV file must contain latitude and longitude columns. The tool automatically detects common column names:

**Latitude**: `lat`, `latitude`, `Latitude`, `LAT`
**Longitude**: `lon`, `lng`, `long`, `longitude`, `Longitude`, `LON`, `LNG`

### Example Input
```csv
name,lat,lon,description
Point A,37.7749,-122.4194,San Francisco
Point B,37.7849,-122.4094,Near SF
Point C,40.7128,-74.0060,New York
```

## Output Format

The output CSV contains all original columns plus a new `cluster` column:
- Positive integers (0, 1, 2, ...): Points belonging to clusters
- `-1`: Noise points (don't belong to any cluster)

### Example Output
```csv
name,lat,lon,description,cluster
Point A,37.7749,-122.4194,San Francisco,0
Point B,37.7849,-122.4094,Near SF,0
Point C,40.7128,-74.0060,New York,-1
```

## Algorithm Parameters

### Epsilon (ε)
- **Definition**: Maximum distance between two points to be considered neighbors
- **Unit**: Kilometers
- **Effect**: Smaller values create tighter, more numerous clusters

### Min Samples
- **Definition**: Minimum number of points required to form a dense region (cluster)
- **Effect**: Higher values require more evidence to form clusters, reducing noise sensitivity

## Visualizing Clusters in Google Earth Pro

After generating cluster data, you can visualize the results with different colors in Google Earth Pro using the following steps:

### Step 1: Prepare Your Data
- Export cluster results to CSV format with columns:
  - `latitude` (or `lat`)
  - `longitude` (or `lon` or `lng`)
  - `cluster_id` (or cluster identifier field)
  - Any additional data fields

### Step 2: Import Data into Google Earth Pro
1. Open Google Earth Pro
2. Go to **File** → **Import**
3. Select your CSV file
4. Configure import settings:
   - Set latitude and longitude field mappings
   - Ensure cluster field is recognized

### Step 3: Apply Color-Coding by Cluster
1. **Access Properties**:
   - Right-click on your imported data layer in the Places panel
   - Select **Properties**

2. **Configure Style & Color**:
   - Go to **Style, Color** tab
   - Select **"Set color from field"**
   - Choose your cluster field from the dropdown

3. **Set Color Options**:
   - **For Numeric Clusters** (e.g., 1, 2, 3):
     - Choose number of buckets (color groups)
     - Set starting and ending colors
     - Google Earth Pro automatically calculates intermediate colors
   - **For Text Clusters** (e.g., "cluster_A", "cluster_B"):
     - Each unique value gets a different color
     - Use "Random colors" for automatic assignment

### Step 4: Customize Visualization
- **Reverse Order**: Click to reverse color assignments if needed
- **Create Subfolders**: Organize clusters into folders for better management
- **Save Template**: Save your color scheme for future use

### Color Mapping Options
- **Single Color**: All points same color
- **Random Colors**: Automatic variety of colors  
- **Field-Based Colors**: Colors based on cluster values (recommended for clusters)

### Tips for Better Visualization
- Use contrasting colors for clear cluster distinction
- Create subfolders to show/hide specific clusters
- Save successful color schemes as templates for reuse
- For large datasets (>32K points), consider splitting into multiple files

### Alternative: Google My Maps
For simpler visualizations, you can also use Google My Maps:
1. Import your CSV file
2. Style data by cluster field
3. Share or embed the resulting map