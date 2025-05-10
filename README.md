# JU E-Cell Coordinator Recruitment Portal [2025]
- Implementation incorporating all requirements:
 ## Techstack Used:
 - Rust
 - Tailwind CSS via CDN
 - Used gtag for analytics, and JSON control on backend

 ### Features:
  [x] JWT authentication
  [x] Pre-loaded applicant data
  [x] Mobile/desktop responsive design
  [x] Tailwind CSS via CDN


 ## System Architecture:
 ```
 Frontend (Yew/WASM)  <--HTTP-->  Backend (Actix-Web)  <-->  data.json
       ↑                                  ↑
    JWT Auth                          Token Validation
```
 ## Backend Implementation:
 backend/
├── src/
│   ├── main.rs
│   ├── auth/
│   │   ├── models.rs
│   │   ├── middleware.rs
│   │   └── service.rs
│   └── applicants/
│       └── service.rs
├── data/
│   └── data.json
└── Cargo.toml

 ## Frontend Implementation:
 frontend/
├── src/
│   ├── main.rs
│   ├── auth/
│   │   └── context.rs
│   ├── components/
│   │   ├── login.rs
│   │   ├── applicant_card.rs
│   │   ├── applicant_list.rs
│   │   └── header.rs
│   ├── models/
│   │   └── applicant.rs
│   ├── services/
│   │   └── api.rs
│   └── router.rs
├── static/
│   ├── index.html
│   └── data/
│       └── data.json
└── Cargo.toml
 ## Deployment Instructions
  - Set up backend environment:

```
cd backend
echo "JWT_SECRET=your_very_secure_secret_here" > .env
echo "ADMIN_USERNAME=admin" >> .env
echo "ADMIN_PASSWORD_HASH=$(cargo run --bin hash_password)" >> .env
```

 - Prepare frontend:

```
cd ../frontend
trunk build --release
```
 - Run the system:

Backend: cargo run (from backend directory)

Frontend: Serve the dist folder from any web server