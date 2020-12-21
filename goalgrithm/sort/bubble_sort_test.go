package sort

import "testing"

func TestBubbleSort(t *testing.T) {
	type args struct {
		src []int
	}
	tests := []struct {
		name string
		args args
	}{
		// TODO: Add test cases.
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			BubbleSort(tt.args.src)
		})
	}
}


func TestQuickSort(t *testing.T) {
	type args struct {
		src []int
		low int 
		high int
	}
	tests := []struct {
		name string
		args args
	}{
		{
			name: "test1",
			args: args{
				[]int{4,6,100,28,1,0,11,2,53,12,4,21,12321,4,31,12,31,231,23,2,65,6,7,8,5,4,6,97,55},
				0,
				28,
			},

		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			QuickSort(tt.args.src, tt.args.low, tt.args.high)
			println(tt.args.src)
			t.Log(tt.args.src)
			// t.Logf("print %d", tt.args.src)
			// t.Error(tt.args.src)
			
		})
	}
}