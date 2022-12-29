# Simimg 🖼

![demo](./assets/demo.gif)

Simimg is a simple image comparison tool. It can be used to compare two images and output the difference between them.

![Status](https://img.shields.io/cirrus/github/yashraj-n/simimg?style=flat-square)
![License](https://img.shields.io/github/license/yashraj-n/simimg?style=flat-square)

## Installation

### Manual

```bash
git clone https://github.com/yashraj-n/simimg
cd simimg
cargo run
```
---

### Download pre-built binaries

You can download pre-built binaries from the [releases](https://github.com/yashraj-n/simimg/releases) page.

## Functions

---
- PSNR: Peak Signal-to-Noise Ratio ![PSNR](https://latex.codecogs.com/gif.latex?PSNR%20%3D%2010%20%5Clog_%7B10%7D%20%5Cleft%20%28%20%5Cfrac%7B%5Ctext%7BMAX%7D%5E2%7D%7BMSE%7D%20%5Cright%20%29)
---

- RASE: Relative Average Spectral Error ![RASE](https://)
---
- RME: Root Mean Error ![RME](https://latex.codecogs.com/gif.latex?RME%20%3D%20%5Csqrt%7B%5Cfrac%7B1%7D%7BN%7D%20%5Csum_%7Bi%3D1%7D%5E%7BN%7D%20%28I_i%20-%20J_i%29%5E2%7D)
---
- SAM: Spectral Angle Mapper ![SAM](https://latex.codecogs.com/gif.latex?SAM%20%3D%20%5Ccos%5E%7B-1%7D%20%5Cleft%20%28%20%5Cfrac%7B%5Csum_%7Bi%3D1%7D%5E%7BN%7D%20I_i%20J_i%7D%7B%5Csqrt%7B%5Csum_%7Bi%3D1%7D%5E%7BN%7D%20I_i%5E2%7D%20%5Csqrt%7B%5Csum_%7Bi%3D1%7D%5E%7BN%7D%20J_i%5E2%7D%7D%20%5Cright%20%29)
---
- SCC: Spatial Correlation Coefficient ![SCC](https://latex.codecogs.com/gif.latex?SCC%20%3D%20%5Cfrac%7B%5Csum_%7Bi%3D1%7D%5E%7BN%7D%20%28I_i%20-%20%5Cbar%7BI%7D%29%20%28J_i%20-%20%5Cbar%7BJ%7D%29%7D%7B%5Csqrt%7B%5Csum_%7Bi%3D1%7D%5E%7BN%7D%20%28I_i%20-%20%5Cbar%7BI%7D%29%5E2%7D%20%5Csqrt%7B%5Csum_%7Bi%3D1%7D%5E%7BN%7D%20%28J_i%20-%20%5Cbar%7BJ%7D%29%5E2%7D%7D)
---
- SSIM: Structural Similarity Index ![SSIM](https://latex.codecogs.com/gif.latex?SSIM%20%3D%20%5Cfrac%7B%282%20%5Cmu_I%20%5Cmu_J%29%20%5Cleft%20%28%202%20%5Csigma_%7B%5Cmu%7D%20%5Csigma_%7B%5Cmu%7D%20%5Cright%20%29%7D%7B%28%5Cmu_I%5E2%20&plus;%20%5Cmu_J%5E2%29%20%5Cleft%20%28%20%5Csigma_I%5E2%20&plus;%20%5Csigma_J%5E2%20%5Cright%20%29%7D)
---
- UQI: Universal Quality Image Index ![UQI]()
---
- VIF: Visual Information Fidelity ![VIF](https://latex.codecogs.com/gif.latex?VIF%20%3D%20%5Cfrac%7B%5Ctext%7BMAX%7D%5E2%7D%7B%5Csigma%5E2%20%5Cleft%20%28%20%5Cfrac%7B%5Csum_%7Bi%3D1%7D%5E%7BN%7D%20%28I_i%20-%20J_i%29%5E2%7D%7B%5Csum_%7Bi%3D1%7D%5E%7BN%7D%20%28I_i%20-%20%5Cbar%7BI%7D%29%5E2%7D%20%5Cright%20%29%7D)


## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
