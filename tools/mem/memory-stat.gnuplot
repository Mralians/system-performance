set terminal pngcairo size 800,400
input_file = system("echo ${MEM_STAT_DATA}")
output_file = system("basename ${MEM_STAT_DATA} .txt | sed 's/$/\.png/'")

set output output_file
set title 'Memory Usage'
set xlabel 'Time'
set ylabel 'Memory (MiB)'
plot input_file using 1 with linespoints title input_file

