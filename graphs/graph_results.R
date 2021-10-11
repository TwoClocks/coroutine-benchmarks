
library(dplyr)
library(ggplot2)
library(tidyverse)
library(RColorBrewer)
library(grid)
library(extrafont)
library(stringr)
library(cowplot)

font_import()

#cpu_type <- "Intel i7-8700K"
cpu_type <- "AMD Ryzen 5 5600G"

load_data <- function(fileName,name) {
  data <- read.csv2(fileName,header = TRUE, sep=",")
  data <- select(data,sample_measured_value,unit,iteration_count)
  names(data) <- c("sample","unit","ittr_num")
  data$sample <- as.numeric(data$sample)
  data$sample <- data$sample / data$ittr_num
  data$name <- name
  select(data,name,sample)
}


graph_data <- function(data,lang_name) {
  
  grob <- grobTree(
    textGrob(
      "whisker lines are 99% confidence interval\nwhite diamonds are mean",
      x=0.35,  y=0.90, hjust=0,
      gp=gpar(col="black", fontsize=10)
    )
  )
  
  tmp <- str_replace(lang_name,"\\+\\+","pp")
  fname <- paste( tolower(tmp), "-", word(cpu_type,1),".png",sep="")
  
  write(paste("\n fanme = ",fname, "\n"),stdout())
  
  png(fname,res=150,width=800, height=400)
  
  p1 <- ggplot(data, aes(x=name, y=sample, fill=name, color=name)) +
    geom_violin(show.legend = FALSE,width=1.5) +
    coord_flip() +
    scale_fill_brewer(type="div",palette="Dark2") +
    scale_colour_brewer(type="div",palette="Dark2") +
    #stat_summary(fun.data = mean_sdl, fun.args=list(mult=1), geom="crossbar",width=0.08,fill=NA,color="black") +
    stat_summary(fun.data = median_hilow, fun.args=list(conf.int=0.99),geom='errorbar', color="black", width=.2) +
    stat_summary(fun = "mean", geom="point",size=4,shape=23,fill="white",color="black") +
    theme(plot.title = element_text(hjust = 0.5),plot.subtitle = element_text(hjust = 0.5), text = element_text(size = 10)) +
    labs(title=paste(lang_name, " round-trip times"), y="Nanoseconds",x=element_blank(),subtitle=cpu_type)
    #annotation_custom(grob)
  
  p1 <- add_sub(p1, "whisker lines are 99% confidence interval. white diamonds are mean", size=6)
  ggdraw(p1)
  
}


rust_atomic <- load_data("../target/criterion/atomic_spin/rust_atomic/new/raw.csv","Rust")
rust_resume <- load_data("../target/criterion/atomic_spin/rust_async_resume/new/raw.csv","Rust-Resume")
rust_suspend <- load_data("../target/criterion/atomic_spin/rust_async_suspend/new/raw.csv","Rust-Suspend")

all <- rbind( rust_atomic, rust_resume, rust_suspend )

graph_data( all, "Rust" )


zig_atomic <- load_data("../target/criterion/atomic_spin/zig_atomic/new/raw.csv","Zig")
zig_resume <- load_data("../target/criterion/atomic_spin/zig_resume/new/raw.csv","Zig-Resume")
zig_suspend <- load_data("../target/criterion/atomic_spin/zig_suspend/new/raw.csv","Zig-Suspend")

all <- rbind( zig_atomic, zig_resume, zig_suspend )

graph_data( all, "Zig" )

cpp_atomic <- load_data("../target/criterion/atomic_spin/cpp_atomic/new/raw.csv","C++")
cpp_resume <- load_data("../target/criterion/atomic_spin/cpp_resume/new/raw.csv","C++-Resume")
cpp_suspend <- load_data("../target/criterion/atomic_spin/cpp_suspend/new/raw.csv","C++-Suspend")

all <- rbind( cpp_atomic, cpp_resume, cpp_suspend )

graph_data( all, "C++" )


kotlin_atomic <- load_data("../target/criterion/atomic_spin/kotlin_atomic/new/raw.csv","Kotlin")
kotlin_resume <- load_data("../target/criterion/atomic_spin/kotlin_resume/new/raw.csv","Kotlin-Resume")
kotlin_suspend <- load_data("../target/criterion/atomic_spin/kotlin_suspend/new/raw.csv","Kotlin-Suspend ")

all <- rbind( kotlin_atomic, kotlin_resume, kotlin_suspend )

graph_data( all, "Kotlin" )

dev.off()


