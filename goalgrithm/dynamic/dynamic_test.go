package dynamic

import "testing"

func TestBubbleSort(t *testing.T) {
	type args struct {
		src [][]int
	}
	tests := []struct {
		name string
		arg args
	}{
		{
			"格子中苹果测试1",
			args{
				[][]int{
					{1, 2, 3, 4},
					{5, 6, 7, 8},
				},
			},
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result :=  two_dismensional(tt.arg.src)
			if result != 27 {
				t.Errorf("dynamic program compute result %d is not 27", result)
			}	
			
		})
	}
}
