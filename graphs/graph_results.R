
library(dplyr)
library(ggplot2)
library(tidyverse)
library(RColorBrewer)
library(grid)
library(extrafont)

font_import()

cpu_type <- "Intel i7-8700K"
#cpu_type <- "AMD Ryzen 5 5600G"

load_data <- function(fileName,name) {
  data <- read.csv2(fileName,header = TRUE, sep=",")
  data <- select(data,sample_measured_value,unit,iteration_count)
  names(data) <- c("sample","unit","ittr_num")
  data$sample <- as.numeric(data$sample)
  data$sample <- data$sample / data$ittr_num
  data$name <- name
  select(data,name,sample)
}

rust_atomic <- load_data("../target/criterion/atomic_spin/rust_atomic/new/raw.csv","Rust")
zig_atomic <- load_data("../target/criterion/atomic_spin/zig_atomic/new/raw.csv","Zig")
kotlin_atomic <- load_data("../target/criterion/atomic_spin/kotlin_atomic/new/raw.csv","Kotlin")
rust_async <- load_data("../target/criterion/atomic_spin/rust_async/new/raw.csv","Rust-Async")
zig_async <- load_data("../target/criterion/atomic_spin/zig_async/new/raw.csv","Zig-Async")
kotlin_async <- load_data("../target/criterion/atomic_spin/kotlin_async/new/raw.csv","Kotlin-Async")

all <- rbind( rust_atomic,zig_atomic,kotlin_atomic, rust_async, zig_async, kotlin_async )


grob <- grobTree(
  textGrob(
    "whisker lines are 99% confidence interval\nwhite diamonds are mean",
    x=0.6,  y=0.95, hjust=0,
    gp=gpar(col="black", fontsize=11)
  )
)

png("results.png",res=150,width=1536, height=1024)
ggplot(all, aes(x=name, y=sample, fill=name, color=name)) +
  geom_violin(show.legend = FALSE,width=1.5) +
  coord_flip() +
  scale_fill_brewer(type="div",palette="Dark2") +
  scale_colour_brewer(type="div",palette="Dark2") +
  #stat_summary(fun.data = mean_sdl, fun.args=list(mult=1), geom="crossbar",width=0.08,fill=NA,color="black") +
  stat_summary(fun = "mean", geom="point",size=4,shape=23,fill="white",color="black") +
  stat_summary(fun.data = median_hilow, fun.args=list(conf.int=0.99),geom='errorbar', color="black", width=.2) +
  theme(plot.title = element_text(hjust = 0.5),plot.subtitle = element_text(hjust = 0.5)) +
  labs(title="Atomic spin-loop round trip times", y="Nanoseconds",x=element_blank(),subtitle=cpu_type) +
  annotation_custom(grob) #+
  #ylim(NA, 175) 
dev.off()

