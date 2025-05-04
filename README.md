# Rust based Simple Image-Viewer

# Rust Image Viewer

A lightweight desktop image viewer built in Rust. Rotate, save, and view images with simple keyboard shortcuts using the `minifb` and `image` crates.

![Screenshot 2025-05-04 222607](https://github.com/user-attachments/assets/7984ef59-5364-4bc2-bfc8-6fd869ff9e96)
Original Image

![Screenshot 2025-05-04 222618](https://github.com/user-attachments/assets/ee6dc017-d351-4473-a23b-4b524cbcc20c)
Rotate Right

![Screenshot 2025-05-04 222640](https://github.com/user-attachments/assets/015ca4fd-6779-4595-91b6-6de428a4ac56)
Rotate Left

![Screenshot 2025-05-04 222716](https://github.com/user-attachments/assets/aee9841c-5d5f-4463-806d-660bdbef6fde)
Save

---


## 🛠️ Tech Stack

- Rust
- Crates:
  - `minifb`
  - `image`

---

## ✨ Features

- View `.jpeg`, `.png`, and other image formats
- Rotate image clockwise with `R`
- Rotate image counter-clockwise with `L`
- Save the rotated image with `S`
- Exit the viewer with `Q`
- Resizable image window with real-time updates

---

## 🚀 Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/Deekshant661/rust-image-viewer.git
cd rust-image-viewer
code .
```

### 2. Add an Image to Load
'''rust
let mut img = image::open("path/to/your/image.jpg")?;
```
