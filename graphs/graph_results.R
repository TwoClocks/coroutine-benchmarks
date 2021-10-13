
library(dplyr)
library(ggplot2)
library(tidyverse)
library(RColorBrewer)
library(grid)
library(extrafont)
library(stringr)
library(cowplot)




font_import()

cpu_type <- "Intel i7-8700K"
#cpu_type <- "AMD Ryzen 5 5600G"

find_maxy <- function( runs ) {
  rng <- runs %>% group_by( name ) %>% group_map( ~ median_hilow( . ) ) %>% 
    bind_rows()
  ymin <- min(rng$ymin)
  ymax <- max(rng$ymax)
  ret <- ((ymax-ymin)*.01) + ymax
  cat(ymin,ymax,ret,"\n")
  ret
}

find_best <- function(glob,name) {
  fl <- Sys.glob(glob)
  fl <- as.data.frame(fl)
  names(fl) <- c("file_name")
  fl$data <- map(fl$file_name, ~ load_data(.,name))
  v <- bind_rows( map(fl$data,~median_hilow(.$sample)) )
  fl$data[[which.min(v$ymax)]]
}

load_data <- function(fileName,name) {
  fdata <- read.csv2(fileName,header = TRUE, sep=",")
  fdata <- select(fdata,sample_measured_value,unit,iteration_count)
  names(fdata) <- c("sample","unit","ittr_num")
  fdata$sample <- as.numeric(fdata$sample)
  fdata$sample <- fdata$sample / fdata$ittr_num
  fdata$name <- name
  select(fdata,name,sample)
}


graph_data <- function(pdata,lang_name,max_y) {
  
  cat(paste(lang_name," mx: ",max_y,"\n"))
  
  tmp <- str_replace(lang_name,"\\+\\+","pp")
  fname <- paste( tolower(tmp), "-", word(cpu_type,1),".png",sep="")
  
  write(paste("\n fanme = ",fname, "\n"),stdout())
  
  png(fname,res=150,width=800, height=400)
  
  p1 <- ggplot(pdata, aes(x=name, y=sample, fill=name, color=name)) +
    geom_violin(show.legend = FALSE,width=1.5) +
    coord_flip() +
    scale_fill_brewer(type="div",palette="Dark2") +
    scale_colour_brewer(type="div",palette="Dark2") +
    #stat_summary(fun.data = mean_sdl, fun.args=list(mult=1), geom="crossbar",width=0.08,fill=NA,color="black") +
    stat_summary(fun.data = median_hilow, geom='errorbar', color="black", width=.3) +
    stat_summary(fun = "mean", geom="point",size=2,shape=23,fill="white",color="black") +
    theme(plot.title = element_text(hjust = 0.5),plot.subtitle = element_text(hjust = 0.5), text = element_text(size = 10)) +
    labs(title=paste(lang_name, " round-trip times"), y="Nanoseconds",x=element_blank(),subtitle=cpu_type) +
    ylim(NA,max_y)

  p1 <- add_sub(p1, "whisker lines are 95% confidence interval. white diamonds are mean", size=6)
  ggdraw(p1)
  
}

rust_atomic <- find_best("../runs/rust_atomic*.csv","Rust")
rust_resume <- find_best("../runs/rust_async_resume*.csv","Rust-Resume")
rust_suspend <- find_best("../runs/rust_async_suspend*.csv","Rust-Suspend")
rust_callback <- find_best("../runs/rust_callback*.csv","Rust-Callback")

all <- rbind( rust_atomic, rust_resume, rust_suspend, rust_callback )

graph_data( all, "Rust", find_maxy( all ) ) 


zig_atomic <- find_best("../runs/zig_atomic*.csv","Zig")
zig_resume <- find_best("../runs/zig_resume*.csv","Zig-Resume")
zig_suspend <- find_best("../runs/zig_suspend*.csv","Zig-Suspend")
zig_callback <- find_best("../runs/zig_callback*.csv","Zig-Callback")

all <- rbind( zig_atomic, zig_resume, zig_suspend, zig_callback )

graph_data( all, "Zig", find_maxy( all ) ) 

cpp_atomic <- find_best("../runs/cpp_atomic*.csv","C++")
cpp_resume <- find_best("../runs/cpp_resume*.csv","C++-Resume")
cpp_suspend <- find_best("../runs/cpp_suspend*.csv","C++-Suspend")
cpp_callback <- find_best("../runs/cpp_suspend*.csv","C++-Callback")

all <- rbind( cpp_atomic, cpp_resume, cpp_suspend, cpp_callback )

graph_data( all, "C++", find_maxy( all ) ) 


kotlin_atomic <- find_best("../runs/kotlin_atomic*.csv","Kotlin")
kotlin_resume <- find_best("../runs/kotlin_resume*.csv","Kotlin-Resume")
kotlin_suspend <- find_best("../runs/kotlin_suspend*.csv","Kotlin-Suspend ")
kotlin_callback <- find_best("../runs/kotlin_callback*.csv","Kotlin-Callback ")

all <- rbind( kotlin_atomic, kotlin_resume, kotlin_suspend, kotlin_callback )

graph_data( all, "Kotlin", find_maxy( all ) ) 

dev.off()


