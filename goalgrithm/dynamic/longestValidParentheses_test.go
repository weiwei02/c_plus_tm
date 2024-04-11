package dynamic

import "testing"

func Test_longestValidParentheses(t *testing.T) {
	tests := []struct {
		name string
		want int
	}{
		{
			"()(())",
			6,
		},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			if got := longestValidParentheses(tt.name); got != tt.want {
				t.Errorf("longestValidParentheses() = %v, want %v", got, tt.want)
			}
		})
	}
}
