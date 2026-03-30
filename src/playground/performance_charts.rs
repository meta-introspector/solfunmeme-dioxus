use dioxus::prelude::*;
use dioxus_charts::{BarChart, LineChart, PieChart};

#[component]
pub fn PerformanceCharts() -> Element {
    rsx! {
            div {
                style: "padding: 20px; max-width: 1200px; margin: 0 auto;",

                h1 {
                    style: "color: #333; margin-bottom: 30px; text-align: center;",
                    "Performance Data Charts"
                }

                // CPU Usage Over Time - Line Chart
                div {
                    style: "margin-bottom: 40px;",
                    h2 {
                        style: "color: #555; margin-bottom: 15px;",
                        "CPU Usage Over Time"
                    }
                    div {
                        style: "background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                        LineChart {
                            padding_top: 30,
                            padding_left: 65,
                            padding_right: 80,
                            padding_bottom: 30,
                            label_interpolation: (|v| format!("{v:.0}%")) as fn(f32) -> String,
                            series: vec![
                                vec![25.0, 30.5, 45.2, 38.7, 52.1, 48.3, 35.8, 42.9, 55.2, 47.8],
                                vec![15.0, 18.2, 22.1, 19.5, 25.3, 23.1, 17.8, 21.4, 28.7, 24.2],
                            ],
                            labels: vec![
                                "00:00".into(), "02:00".into(), "04:00".into(), "06:00".into(), "08:00".into(),
                                "10:00".into(), "12:00".into(), "14:00".into(), "16:00".into(), "18:00".into(),
                            ],
                            series_labels: vec!["CPU Core 1".into(), "CPU Core 2".into()],
                            viewbox_width: 800,
                            viewbox_height: 300,
                        }
                    }
                }

                // Memory Usage by Process - Bar Chart
                div {
                    style: "margin-bottom: 40px;",
                    h2 {
                        style: "color: #555; margin-bottom: 15px;",
                        "Memory Usage by Process"
                    }
                    div {
                        style: "background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                        BarChart {
                            padding_top: 30,
                            padding_left: 70,
                            padding_right: 50,
                            padding_bottom: 30,
                            bar_width: "8%",
                            horizontal_bars: true,
                            label_interpolation: (|v| format!("{v:.1} MB")) as fn(f32) -> String,
                            series: vec![
                                vec![512.5, 256.3, 128.7, 89.2, 64.1, 45.8, 32.4, 28.9],
                            ],
                            labels: vec![
                                "Chrome".into(), "VS Code".into(), "Node.js".into(), "PostgreSQL".into(),
                                "Docker".into(), "Slack".into(), "Spotify".into(), "Discord".into(),
                            ],
                            viewbox_width: 800,
                            viewbox_height: 400,
                        }
                    }
                }

                // Network Traffic - Stacked Bar Chart
                div {
                    style: "margin-bottom: 40px;",
                    h2 {
                        style: "color: #555; margin-bottom: 15px;",
                        "Network Traffic by Protocol"
                    }
                    div {
                        style: "background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                        BarChart {
                            padding_top: 30,
                            padding_left: 70,
                            padding_right: 50,
                            padding_bottom: 30,
                            bar_width: "12%",
                            stacked_bars: true,
                            label_interpolation: (|v| format!("{v:.0} MB")) as fn(f32) -> String,
                            series: vec![
                                vec![45.2, 38.7, 52.1, 48.3, 35.8, 42.9], // HTTP
                                vec![12.3, 15.8, 18.2, 14.7, 11.9, 16.4], // HTTPS
                                vec![8.7, 6.2, 9.1, 7.8, 5.4, 8.9],      // FTP
                            ],
                            labels: vec![
                                "Mon".into(), "Tue".into(), "Wed".into(), "Thu".into(), "Fri".into(), "Sat".into(),
                            ],
                            //labels: vec!["HTTP".into(), "HTTPS".into(), "FTP".into()],
                            viewbox_width: 800,
                            viewbox_height: 300,
                        }
                    }
                }

                // Disk Usage - Pie Chart
                div {
                    style: "margin-bottom: 40px;",
                    h2 {
                        style: "color: #555; margin-bottom: 15px;",
                        "Disk Usage Distribution"
                    }
                    div {
                        style: "background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                        PieChart {
                            series: vec![
                    45.2,
                                // ("System Files".into(), 45.2),
                                // ("Applications".into(), 28.7),
                                // ("User Data".into(), 15.8),
                                // ("Downloads".into(), 8.3),
                                // ("Cache".into(), 2.0),
                            ],
                            viewbox_width: 400,
                            viewbox_height: 400,
                            padding: 20.0,
    //                        padding_bottom: 20,
    //                        padding_left: 20,
    //                        padding_right: 20,
                        }
                    }
                }

                // Response Time Trends - Line Chart
                div {
                    style: "margin-bottom: 40px;",
                    h2 {
                        style: "color: #555; margin-bottom: 15px;",
                        "API Response Time Trends"
                    }
                    div {
                        style: "background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                        LineChart {
                            padding_top: 30,
                            padding_left: 65,
                            padding_right: 80,
                            padding_bottom: 30,
                            label_interpolation: (|v| format!("{v:.0}ms")) as fn(f32) -> String,
                            series: vec![
                                vec![120.5, 135.2, 98.7, 145.3, 112.8, 89.4, 156.7, 103.2, 78.9, 134.5],
                                vec![85.2, 92.1, 76.8, 108.4, 89.7, 67.3, 123.8, 94.1, 62.5, 98.7],
                            ],
                            labels: vec![
                                "9:00".into(), "10:00".into(), "11:00".into(), "12:00".into(), "13:00".into(),
                                "14:00".into(), "15:00".into(), "16:00".into(), "17:00".into(), "18:00".into(),
                            ],
                            series_labels: vec!["Database Queries".into(), "API Calls".into()],
                            viewbox_width: 800,
                            viewbox_height: 300,
                        }
                    }
                }

                // Error Rates - Bar Chart
                div {
                    style: "margin-bottom: 40px;",
                    h2 {
                        style: "color: #555; margin-bottom: 15px;",
                        "Error Rates by Service"
                    }
                    div {
                        style: "background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                        BarChart {
                            padding_top: 30,
                            padding_left: 70,
                            padding_right: 50,
                            padding_bottom: 30,
                            bar_width: "10%",
                            label_interpolation: (|v| format!("{v:.2}%")) as fn(f32) -> String,
                            series: vec![
                                vec![0.15, 0.08, 0.23, 0.05, 0.12, 0.19, 0.07, 0.31],
                            ],
                            labels: vec![
                                "Auth Service".into(), "Payment API".into(), "User Service".into(), "Email Service".into(),
                                "File Storage".into(), "Analytics".into(), "Notification".into(), "Search API".into(),
                            ],
                            viewbox_width: 800,
                            viewbox_height: 300,
                        }
                    }
                }
            }
        }
}
