import { format } from 'date-fns';

export function Dateformat(date, formatString) {
    return format(date, formatString);
}