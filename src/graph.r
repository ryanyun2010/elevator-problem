library(tidyverse)

results <- read_csv("results.csv")

print(head(results))
print(nrow(results))

best_by_mean <- results %>%
  group_by(arrival_time_mean) %>%
  slice_min(difference_to_target, n = 1, with_ties = FALSE) %>%
  ungroup()

overall_best <- results %>%
  slice_min(difference_to_target, n = 1, with_ties = FALSE)

cat("\nBEST OVERALL RESULT\n")
print(overall_best)

p1 <- ggplot(best_by_mean, aes(x = arrival_time_mean, y = arrival_time_std_dev)) +
  geom_line(color = "steelblue", linewidth = 1) +
  geom_point(color = "steelblue", size = 1.2) +
  geom_point(
    data = overall_best,
    aes(x = arrival_time_mean, y = arrival_time_std_dev),
    color = "red",
    size = 4
  ) +
  geom_label(
    data = overall_best,
    aes(
      x = arrival_time_mean,
      y = arrival_time_std_dev,
      label = paste0(
        "Best\nMean = ", arrival_time_mean,
        "\nSD = ", arrival_time_std_dev,
        "\nDifference = ", round(difference_to_target, 3)
      )
    ),
    color = "red",
    fill = "white",
    fontface = "bold",
    nudge_y = 40
  ) +
  labs(
    title = "Best Standard Deviation for Each Arrival-Time Mean",
    subtitle = "Each point is the SD closest to a mean late time of 110",
    x = "Arrival Time Mean",
    y = "Best Arrival Time Standard Deviation"
  ) +
  theme_minimal(base_size = 13) +
  theme(
    plot.title = element_text(face = "bold"),
    panel.grid.minor = element_blank()
  )

print(p1)

p2 <- ggplot(best_by_mean, aes(x = arrival_time_mean, y = difference_to_target)) +
  geom_line(color = "darkorange", linewidth = 1) +
  geom_point(color = "darkorange", size = 1.2) +
  geom_point(
    data = overall_best,
    aes(x = arrival_time_mean, y = difference_to_target),
    color = "red",
    size = 4
  ) +
  labs(
    title = "Distance From Target Mean of 110",
    subtitle = "Lower values are better",
    x = "Arrival Time Mean",
    y = "Absolute Difference From 110"
  ) +
  theme_minimal(base_size = 13) +
  theme(
    plot.title = element_text(face = "bold"),
    panel.grid.minor = element_blank()
  )

print(p2)

top_20 <- results %>%
  arrange(difference_to_target) %>%
  slice_head(n = 20)

cat("TOP 20 CLOSEST SIMULATIONS\n")
print(top_20)